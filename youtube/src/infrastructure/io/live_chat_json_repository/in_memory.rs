use crate::domain::live_chat::{repository::FetchLiveChatRepository, LiveChatEntity};
use anyhow::Context as _;
use std::io::{BufRead as _, BufReader, Cursor, Read as _};

pub(crate) struct InMemoryRepository {
    str: String,
}

impl From<String> for InMemoryRepository {
    fn from(value: String) -> Self {
        Self { str: value }
    }
}

impl FetchLiveChatRepository for InMemoryRepository {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>> {
        let mut live_chats = Vec::new();

        let cursor = Cursor::new(&self.str);
        let buffered = BufReader::new(cursor);

        for (line_number, line) in buffered.lines().enumerate() {
            let line_number = line_number + 1;
            let line = line?;

            let live_chat = serde_json::from_str::<LiveChatEntity>(&line)
                .with_context(|| format!("Failed to convert at {line_number}"))?;

            live_chats.push(live_chat);
        }

        Ok(live_chats)
    }

    fn one_chunk(&self) -> anyhow::Result<LiveChatEntity> {
        let mut content = String::new();

        let cursor = Cursor::new(&self.str);
        let mut buf = BufReader::new(cursor);
        buf.read_to_string(&mut content)?;

        let live_chat = serde_json::from_str::<LiveChatEntity>(&content)
            .with_context(|| format!("Failed to convert the content"))?;

        Ok(live_chat)
    }
}
