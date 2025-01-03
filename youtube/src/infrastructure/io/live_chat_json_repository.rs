pub(crate) mod file;
pub(crate) mod in_memory;

use crate::domain::live_chat::{repository::FetchLiveChatRepository, LiveChatEntity};
use file::FileRepository;
use in_memory::InMemoryRepository;
use std::path::PathBuf;

pub struct IoLiveChatRepository<T> {
    inner: T,
}

impl From<PathBuf> for IoLiveChatRepository<FileRepository> {
    fn from(value: PathBuf) -> Self {
        let inner = FileRepository::from(value);
        Self { inner }
    }
}

impl From<String> for IoLiveChatRepository<InMemoryRepository> {
    fn from(value: String) -> Self {
        let inner = InMemoryRepository::from(value);
        Self { inner }
    }
}

impl<T: FetchLiveChatRepository> FetchLiveChatRepository for IoLiveChatRepository<T> {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>> {
        self.inner.all()
    }

    fn one_chunk(&self) -> anyhow::Result<LiveChatEntity> {
        self.inner.one_chunk()
    }
}
