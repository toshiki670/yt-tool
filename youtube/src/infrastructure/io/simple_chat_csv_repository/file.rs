use crate::domain::simple_chat::{repository::SaveSimpleChatRepository, SimpleChatEntity};
use anyhow::Context as _;
use std::{fs::File, path::PathBuf};

pub(crate) struct FileRepository {
    path: PathBuf,
}

impl From<PathBuf> for FileRepository {
    fn from(value: PathBuf) -> Self {
        Self { path: value }
    }
}

impl SaveSimpleChatRepository for FileRepository {
    fn bulk_create(&mut self, simple_chats: Vec<SimpleChatEntity>) -> anyhow::Result<()> {
        let file = File::create(&self.path)
            .with_context(|| format!("Failed to create {}", &self.path.display()))?;
        let mut wtr = csv::Writer::from_writer(file);

        for simple_chat in simple_chats {
            wtr.serialize(&simple_chat)
                .with_context(|| format!("Failed to serialize at {:?}", &simple_chat))?;
        }
        wtr.flush().context("Failed to flush")?;

        Ok(())
    }
}
