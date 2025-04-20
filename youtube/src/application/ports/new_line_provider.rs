use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(thiserror::Error, Debug)]
pub enum ProviderError {
    #[error("Notify error: {0}")]
    Notify(#[from] notify::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Channel disconnected")]
    ChannelDisconnected,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// ファイルから新しい行を非同期に提供するトレイト (ポート)
#[async_trait]
pub trait NewLineProvider: Send + Sync {
    /// 指定されたファイルを監視し、新しい行のストリームを返す
    async fn watch_new_lines(
        &self,
        file_path: &Path,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, ProviderError>> + Send>>, ProviderError>;
}
