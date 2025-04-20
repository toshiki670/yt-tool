use crate::application::ports::new_line_provider::{NewLineProvider, ProviderError};
use crate::application::ports::output_presenter::OutputPresenter;
use crate::domain::live_chat::LiveChatEntity; // 自身のクレート内の domain を参照
use crate::domain::simple_chat::SimpleChatEntity;
use anyhow::{Context, Result};
use futures::stream::{self, Stream, StreamExt, TryStreamExt};
use log::{error, info, warn};
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use tokio::task::spawn_blocking;

#[derive(thiserror::Error, Debug)]
pub enum WatchChatError {
    #[error("Provider error: {0}")]
    Provider(#[from] ProviderError),
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Domain conversion error: {0}")]
    DomainConversion(anyhow::Error),
    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Presenter error: {0}")]
    Presenter(anyhow::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// チャット監視ユースケース
pub struct WatchChatUseCase<P, O>
where
    P: NewLineProvider + Send + Sync + 'static,
    O: OutputPresenter + Send + Sync + 'static,
{
    line_provider: Arc<P>,
    presenter: Arc<O>,
}

impl<P, O> WatchChatUseCase<P, O>
where
    P: NewLineProvider + Send + Sync + 'static,
    O: OutputPresenter + Send + Sync + 'static,
{
    pub fn new(line_provider: Arc<P>, presenter: Arc<O>) -> Self {
        Self {
            line_provider,
            presenter,
        }
    }

    /// ユースケースを実行し、チャットの監視とプレゼンテーションを行う
    ///
    /// # Errors
    ///
    /// - ファイル監視の初期化に失敗した場合
    /// - ファイルの読み取り中にエラーが発生した場合
    /// - ファイルが存在しなくなった場合
    /// - 内部タスクエラーが発生した場合
    pub async fn execute(&self, file_path: &Path) -> Result<(), WatchChatError> {
        info!("Starting chat watch use case for: {}", file_path.display());

        // 監視開始メッセージの表示
        self.presenter
            .present_message(format!(
                "Watching for new messages in: {}",
                file_path.display()
            ))
            .await
            .map_err(WatchChatError::Presenter)?;

        // チャットストリームを取得
        let chat_stream = match self.get_chat_stream(file_path).await {
            Ok(stream) => stream,
            Err(e) => {
                error!("Failed to initialize chat watching stream: {e:?}");

                // エラーメッセージ表示
                self.presenter
                    .present_error(format!(
                        "Failed to start watching file: {}",
                        file_path.display()
                    ))
                    .await
                    .map_err(WatchChatError::Presenter)?;

                return Err(e);
            }
        };

        // チャットストリームを処理
        self.process_chat_stream(chat_stream, file_path).await
    }

    /// チャットエンティティのストリームを取得する内部メソッド
    async fn get_chat_stream(
        &self,
        file_path: &Path,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<SimpleChatEntity, WatchChatError>> + Send>>,
        WatchChatError,
    > {
        let line_stream = self.line_provider.watch_new_lines(file_path).await?;

        let processed_stream = line_stream
            .map_err(WatchChatError::from)
            .and_then(|line| async move {
                // 空行チェック
                if line.trim().is_empty() {
                    return Ok(stream::iter(vec![]).map(Ok));
                }

                // JSONのパース処理だけをspawn_blockingで実行
                let live_chat: LiveChatEntity = spawn_blocking(move || {
                    serde_json::from_str(&line).context("Failed to parse JSON line")
                })
                .await??;

                // パース後の変換処理は非同期メソッドを使用
                let simple_chats = live_chat
                    .try_into_simple_chats()
                    .await
                    .map_err(|e| anyhow::anyhow!(e).context("Failed during domain conversion"))?;

                Ok(stream::iter(simple_chats).map(Ok))
            })
            .try_flatten();

        Ok(Box::pin(processed_stream))
    }

    /// チャットストリームを処理し、結果をプレゼンターに渡す内部メソッド
    async fn process_chat_stream(
        &self,
        mut chat_stream: Pin<
            Box<dyn Stream<Item = Result<SimpleChatEntity, WatchChatError>> + Send>,
        >,
        file_path: &Path,
    ) -> Result<(), WatchChatError> {
        loop {
            match chat_stream.try_next().await {
                Ok(Some(simple_chat)) => {
                    // SimpleChatEntity をプレゼンターに渡して表示
                    self.presenter
                        .present_chat(&simple_chat)
                        .await
                        .map_err(WatchChatError::Presenter)?;
                }
                Ok(None) => {
                    // ストリームが正常に終了
                    let message = format!(
                        "Chat stream finished successfully for: {}",
                        file_path.display()
                    );
                    info!("{message}");
                    self.presenter
                        .present_message(message)
                        .await
                        .map_err(WatchChatError::Presenter)?;
                    break; // ループを抜ける
                }
                Err(e) => {
                    // ストリーム処理中にエラー発生
                    warn!("Error encountered in chat stream: {e:?}");

                    // リカバリー不能なエラーの場合は早期リターン
                    match &e {
                        WatchChatError::Provider(provider_error) => {
                            use crate::application::ports::new_line_provider::ProviderError;
                            if let ProviderError::Io(io_err) = provider_error {
                                if io_err.kind() == std::io::ErrorKind::NotFound {
                                    let err_msg = "Watched file not found or disappeared";
                                    error!("{err_msg}");
                                    self.presenter
                                        .present_error(err_msg)
                                        .await
                                        .map_err(WatchChatError::Presenter)?;
                                    return Err(e);
                                }
                            }
                        }
                        WatchChatError::JoinError(_) => {
                            let err_msg = "Internal task error occurred";
                            error!("{err_msg}");
                            self.presenter
                                .present_error(err_msg)
                                .await
                                .map_err(WatchChatError::Presenter)?;
                            return Err(e);
                        }
                        // 他のエラー (パースエラー等) はログ出力して継続
                        _ => {
                            let warn_msg = format!("Recoverable error, continuing watch: {e}");
                            warn!("{warn_msg}");
                            self.presenter
                                .present_error(warn_msg)
                                .await
                                .map_err(WatchChatError::Presenter)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
