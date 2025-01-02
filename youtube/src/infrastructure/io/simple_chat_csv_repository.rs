pub(crate) mod file;

use crate::domain::simple_chat::{repository::SaveSimpleChatRepository, SimpleChatEntity};
use anyhow::Context as _;
use std::io::Write;

pub struct IoSimpleChatRepository<T> {
    inner: T,
}

impl<W: Write> IoSimpleChatRepository<W> {
    pub fn create(inner: W) -> anyhow::Result<Self> {
        Ok(Self { inner })
    }
}

impl<W: Write> SaveSimpleChatRepository for IoSimpleChatRepository<W>
where
    for<'a> &'a W: Write,
{
    fn bulk_create(&self, simple_chats: Vec<SimpleChatEntity>) -> anyhow::Result<()> {
        let mut wtr = csv::Writer::from_writer(&self.inner);

        for simple_chat in simple_chats {
            wtr.serialize(&simple_chat)
                .with_context(|| format!("Failed to serialize at {:?}", &simple_chat))?;
        }
        wtr.flush().context("Failed to flush")?;

        Ok(())
    }
}
