use crate::domain::live_chat::{repository::FetchLiveChatRepository, LiveChatEntity};
use anyhow::Context as _;
use std::{
    fs::File,
    io::{BufRead as _, BufReader, Read as _},
    path::PathBuf,
};

pub(crate) struct FileRepository {
    path: PathBuf,
}

impl From<PathBuf> for FileRepository {
    fn from(value: PathBuf) -> Self {
        Self { path: value }
    }
}

impl FetchLiveChatRepository for FileRepository {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>> {
        let mut live_chats = Vec::new();

        let file = File::open(&self.path).with_context(|| format!("Failed to open {}", &self.path.display()))?;
        let buffered = BufReader::new(file);

        for (line_number, line) in buffered.lines().enumerate() {
            let line_number = line_number + 1;
            let line = line?;

            let live_chat = serde_json::from_str::<LiveChatEntity>(&line).with_context(|| {
                format!(
                    "Failed to convert at {}:{}",
                    self.path.display(),
                    line_number
                )
            })?;

            live_chats.push(live_chat);
        }

        Ok(live_chats)
    }

    fn one_chunk(&self) -> anyhow::Result<LiveChatEntity> {
        let mut content = String::new();

        let file = File::open(&self.path).with_context(|| format!("Failed to open {}", &self.path.display()))?;
        let mut buf = BufReader::new(file);
        buf.read_to_string(&mut content)?;

        let live_chat = serde_json::from_str::<LiveChatEntity>(&content)
            .with_context(|| format!("Failed to convert the content"))?;

        Ok(live_chat)
    }
}
