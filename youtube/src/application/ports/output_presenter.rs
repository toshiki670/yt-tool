use crate::domain::simple_chat::SimpleChatEntity;
use async_trait::async_trait;
use std::fmt::Display;

/// 出力プレゼンターのトレイト
///
/// アプリケーション層からの出力を表示するインターフェース
#[async_trait]
pub trait OutputPresenter: Send + Sync {
    /// メッセージを表示する
    async fn present_message<T: Display + Send>(&self, message: T) -> anyhow::Result<()>;

    /// エラーメッセージを表示する
    async fn present_error<T: Display + Send>(&self, error: T) -> anyhow::Result<()>;

    /// チャットエンティティを表示する
    async fn present_chat(&self, chat: &SimpleChatEntity) -> anyhow::Result<()>;
}
