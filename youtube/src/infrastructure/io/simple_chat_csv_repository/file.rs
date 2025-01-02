use super::IoSimpleChatRepository;
use crate::domain::simple_chat::{repository::SaveSimpleChatRepository, SimpleChatEntity};
use anyhow::Context as _;
use std::{fs::File, path::PathBuf};

pub(crate) struct FileSimpleChatRepository {
    inner: IoSimpleChatRepository<File>,
}

impl FileSimpleChatRepository {
    pub fn create(path: &PathBuf) -> anyhow::Result<Self> {
        let file =
            File::create(path).with_context(|| format!("Failed to create {}", &path.display()))?;
        let inner = IoSimpleChatRepository::create(file)?;
        Ok(Self { inner })
    }
}

impl SaveSimpleChatRepository for FileSimpleChatRepository {
    fn bulk_create(&self, simple_chats: Vec<SimpleChatEntity>) -> anyhow::Result<()> {
        self.inner.bulk_create(simple_chats)
    }
}
