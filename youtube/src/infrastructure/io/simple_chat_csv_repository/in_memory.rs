use crate::domain::simple_chat::{repository::SaveSimpleChatRepository, SimpleChatEntity};
use anyhow::Context as _;
use std::io::Cursor;

pub(crate) struct InMemoryRepository {
    inner: Vec<u8>,
}

impl From<Vec<u8>> for InMemoryRepository {
    fn from(value: Vec<u8>) -> Self {
        Self { inner: value }
    }
}

impl SaveSimpleChatRepository for InMemoryRepository {
    fn bulk_create(&mut self, simple_chats: Vec<SimpleChatEntity>) -> anyhow::Result<()> {
        let cursor = Cursor::new(&mut self.inner);
        let mut wtr = csv::Writer::from_writer(cursor);

        for simple_chat in simple_chats {
            wtr.serialize(&simple_chat)
                .with_context(|| format!("Failed to serialize at {:?}", &simple_chat))?;
        }
        wtr.flush().context("Failed to flush")?;

        Ok(())
    }
}
