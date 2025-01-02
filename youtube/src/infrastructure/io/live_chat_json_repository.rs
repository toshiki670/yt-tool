pub(crate) mod file;

use crate::domain::live_chat::{repository::FetchLiveChatRepository, LiveChatEntity};
use anyhow::Context as _;
use std::io::{BufRead as _, BufReader, Read};
use thiserror::Error;

pub struct IoLiveChatRepository<T> {
    inner: T,
}

#[derive(Error, Debug)]
pub enum IoLiveChatRepositoryError {
    #[error("Failed to convert at {0} row")]
    FailedToConvertInLines(usize),
}

impl<R: Read> IoLiveChatRepository<R> {
    pub fn open(inner: R) -> anyhow::Result<Self> {
        Ok(Self { inner })
    }
}

impl<R: Read> FetchLiveChatRepository for IoLiveChatRepository<R>
where
    for<'a> &'a R: Read,
{
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>> {
        let mut live_chats = Vec::new();

        for (line_number, line) in BufReader::new(&self.inner).lines().enumerate() {
            let line_number = line_number + 1;
            let line = line?;

            let live_chat = serde_json::from_str::<LiveChatEntity>(&line)
                .with_context(|| IoLiveChatRepositoryError::FailedToConvertInLines(line_number))?;

            live_chats.push(live_chat);
        }

        Ok(live_chats)
    }

    fn one_chunk(&self) -> anyhow::Result<LiveChatEntity> {
        let mut content = String::new();
        BufReader::new(&self.inner).read_to_string(&mut content)?;

        let live_chat = serde_json::from_str::<LiveChatEntity>(&content)
            .with_context(|| format!("Failed to convert the content"))?;

        Ok(live_chat)
    }
}
