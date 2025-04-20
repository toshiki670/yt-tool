use crate::application::ports::output_presenter::OutputPresenter;
use crate::application::use_cases::watch_chat::WatchChatError;
use crate::domain::simple_chat::SimpleChatEntity;
use async_trait::async_trait;
use chrono::{self, DateTime, Local};
use log::{error, info, warn};
use std::fmt::Display;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::task::spawn;
use tokio::time::sleep;

/// 標準出力を使用したプレゼンター実装
#[derive(Debug, Clone)]
pub struct StdoutPresenter {
    last_message_time: Arc<Mutex<Option<Instant>>>,
    separator_shown: Arc<Mutex<bool>>,
    message_count: Arc<Mutex<usize>>, // 区切り文字表示後のメッセージカウンター
}

impl Default for StdoutPresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl StdoutPresenter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            last_message_time: Arc::new(Mutex::new(None)),
            separator_shown: Arc::new(Mutex::new(true)), // 初期状態は区切り線表示済み
            message_count: Arc::new(Mutex::new(0)),      // 初期状態は0件
        }
    }

    /// ファイル監視開始メッセージを表示
    pub async fn present_watch_start(&self, file_path: &Path) -> anyhow::Result<()> {
        let message = format!("Watching for new messages in: {}", file_path.display());
        self.present_message(message).await
    }

    /// ファイル監視失敗メッセージを表示
    pub async fn present_watch_start_failure(&self, file_path: &Path) -> anyhow::Result<()> {
        let message = format!("Failed to start watching file: {}", file_path.display());
        self.present_error(message).await
    }

    /// ファイル監視エラーメッセージを処理
    pub async fn handle_watch_error(&self, error: &WatchChatError) -> anyhow::Result<()> {
        use crate::application::ports::new_line_provider::ProviderError;

        match error {
            WatchChatError::Provider(ProviderError::Io(io_err))
                if io_err.kind() == std::io::ErrorKind::NotFound =>
            {
                let err_msg = "Watched file not found or disappeared";
                error!("{err_msg}");
                self.present_error(err_msg).await?;
            }
            WatchChatError::JoinError(_) => {
                let err_msg = "Internal task error occurred";
                error!("{err_msg}");
                self.present_error(err_msg).await?;
            }
            // 他のエラー (パースエラー等) はログ出力して継続
            _ => {
                let warn_msg = format!("Recoverable error, continuing watch: {error}");
                warn!("{warn_msg}");
                self.present_error(warn_msg).await?;
            }
        }

        Ok(())
    }

    /// ファイル監視完了メッセージを表示
    pub async fn present_watch_complete(&self, file_path: &Path) -> anyhow::Result<()> {
        let message = format!(
            "Chat stream finished successfully for: {}",
            file_path.display()
        );
        info!("{message}");
        self.present_message(message).await
    }

    // メッセージを受信したときの処理
    fn handle_message_received(&self) {
        // メッセージカウンターをインクリメント
        {
            let mut message_count = self.message_count.lock().unwrap();
            *message_count += 1;
        }

        // 現在の時刻をセット
        let mut last_message_time = self.last_message_time.lock().unwrap();
        *last_message_time = Some(Instant::now());

        // 区切り線が表示済みか確認
        let mut separator_shown = self.separator_shown.lock().unwrap();
        if *separator_shown {
            // 区切り線が表示済みの場合は、まだ表示されていないとマーク
            *separator_shown = false;
        }

        // 区切り線の表示タイマーを開始
        let last_message_time_clone = self.last_message_time.clone();
        let separator_shown_clone = self.separator_shown.clone();
        let message_count_clone = self.message_count.clone();
        let _handle = spawn(async move {
            // 一定時間（0.5秒）待機
            sleep(Duration::from_millis(500)).await;

            // 最後のメッセージからの経過時間を確認
            let check_time = {
                let last_time = last_message_time_clone.lock().unwrap();
                last_time
                    .map(|t| t.elapsed() >= Duration::from_millis(500))
                    .unwrap_or(false)
            };

            // 一定時間以上経過していて、まだ区切り線が表示されていない場合
            if check_time {
                // メッセージ数を取得
                let message_count = {
                    let count = message_count_clone.lock().unwrap();
                    *count
                };

                // メッセージが8件以上ある場合のみ区切り線を表示
                if message_count >= 8 {
                    let mut separator_shown = separator_shown_clone.lock().unwrap();
                    if !*separator_shown {
                        // 区切り線を表示
                        let now: DateTime<Local> = Local::now();
                        println!(
                            "\n--- File updated with new messages at {} ---\n",
                            now.format("%H:%M:%S")
                        );
                        *separator_shown = true;

                        // メッセージカウンターをリセット
                        let mut message_count = message_count_clone.lock().unwrap();
                        *message_count = 0;
                    }
                }
            }
        });
    }
}

#[async_trait]
impl OutputPresenter for StdoutPresenter {
    /// 通常メッセージを標準出力に表示
    async fn present_message<T: Display + Send>(&self, message: T) -> anyhow::Result<()> {
        println!("{}", message);
        info!("Presented message: {}", message);
        Ok(())
    }

    /// エラーメッセージを標準エラー出力に表示
    async fn present_error<T: Display + Send>(&self, error: T) -> anyhow::Result<()> {
        eprintln!("Error: {}", error);
        error!("Presented error: {}", error);
        Ok(())
    }

    /// SimpleChatEntity を標準出力に表示
    async fn present_chat(&self, chat: &SimpleChatEntity) -> anyhow::Result<()> {
        // フォーマット処理
        let output_string = format_chat_for_display(chat);

        // メッセージを表示
        println!("{}", output_string);

        // メッセージ受信を処理（タイマー開始など）
        self.handle_message_received();

        info!("Presented chat entity: {}", output_string);
        Ok(())
    }
}

/// チャットエンティティをフォーマットする内部関数
fn format_chat_for_display(chat: &SimpleChatEntity) -> String {
    format!(
        "[{}] [{}] {} > {}",
        chat.posted_at
            .map_or_else(|| "-".to_string(), |dt| dt.to_string()),
        chat.category,
        chat.author_name.as_deref().unwrap_or("Unknown"),
        chat.content
    )
}
