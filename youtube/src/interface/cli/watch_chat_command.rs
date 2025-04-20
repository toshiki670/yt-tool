use crate::application::use_cases::watch_chat::WatchChatUseCase;
use crate::infrastructure::file_watcher::FileWatcherAdapter;
use crate::interface::presenter::StdoutPresenter;
use anyhow::Result;
use log::info;
use std::path::Path;
use std::sync::Arc;

/// ファイル監視コマンドハンドラ
///
/// CLI からファイル監視コマンドを受け取り、実行するハンドラ
pub struct WatchChatCommand {
    watch_chat_use_case: Arc<WatchChatUseCase<FileWatcherAdapter, StdoutPresenter>>,
}

impl Default for WatchChatCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchChatCommand {
    /// 標準のFileWatcherとStdoutPresenterを使用するWatchChatCommandを作成
    pub fn new() -> Self {
        // デフォルトのアダプタとプレゼンターを作成
        let file_watcher_adapter = Arc::new(FileWatcherAdapter);
        let stdout_presenter = Arc::new(StdoutPresenter::new());

        // ユースケースを生成
        let watch_chat_use_case = Arc::new(WatchChatUseCase::new(
            file_watcher_adapter,
            stdout_presenter,
        ));

        Self {
            watch_chat_use_case,
        }
    }

    /// ファイルを監視し、新しいチャットメッセージを表示するコマンドを実行
    ///
    /// # Errors
    ///
    /// - ファイル監視の初期化に失敗した場合
    /// - ファイルの読み取り中にエラーが発生した場合
    /// - ファイルが存在しなくなった場合
    /// - 内部タスクエラーが発生した場合
    pub async fn execute(&self, file_path: &Path) -> Result<()> {
        info!(
            "Starting watch_chat command for file: {}",
            file_path.display()
        );

        // ユースケースに処理を委譲
        match self.watch_chat_use_case.execute(file_path).await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }
}
