pub(crate) mod file;
pub(crate) mod in_memory;

use crate::domain::simple_chat::{repository::SaveSimpleChatRepository, SimpleChatEntity};
use file::FileRepository;
use in_memory::InMemoryRepository;
use std::path::PathBuf;

pub struct IoSimpleChatRepository<T> {
    inner: T,
}

impl From<PathBuf> for IoSimpleChatRepository<FileRepository> {
    fn from(value: PathBuf) -> Self {
        let inner = FileRepository::from(value);
        Self { inner }
    }
}

impl From<Vec<u8>> for IoSimpleChatRepository<InMemoryRepository> {
    fn from(value: Vec<u8>) -> Self {
        let inner = InMemoryRepository::from(value);
        Self { inner }
    }
}

impl<T: SaveSimpleChatRepository> SaveSimpleChatRepository for IoSimpleChatRepository<T> {
    fn bulk_create(&mut self, chats: Vec<SimpleChatEntity>) -> anyhow::Result<()> {
        self.inner.bulk_create(chats)
    }
}
