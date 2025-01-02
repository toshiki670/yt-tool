use super::{IoLiveChatRepository, IoLiveChatRepositoryError};
use crate::domain::live_chat::{repository::FetchLiveChatRepository, LiveChatEntity};
use anyhow::Context as _;
use std::{fs::File, path::PathBuf};

pub(crate) struct FileLiveChatRepository {
    inner: IoLiveChatRepository<File>,
    path: PathBuf,
}

impl FileLiveChatRepository {
    pub fn open(path: &PathBuf) -> anyhow::Result<Self> {
        let file =
            File::open(path).with_context(|| format!("Failed to open {}", &path.display()))?;
        let inner = IoLiveChatRepository::open(file)?;
        Ok(Self {
            inner,
            path: path.clone(),
        })
    }
}

impl FetchLiveChatRepository for FileLiveChatRepository {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>> {
        match self.inner.all() {
            Ok(live_chats) => Ok(live_chats),
            Err(e) => {
                if let Some(IoLiveChatRepositoryError::FailedToConvertInLines(line_number)) =
                    e.downcast_ref::<IoLiveChatRepositoryError>()
                {
                    anyhow::bail!(
                        "Failed to convert at {}:{}",
                        self.path.display(),
                        line_number
                    );
                } else {
                    Err(e)
                }
            }
        }
    }

    fn one_chunk(&self) -> anyhow::Result<LiveChatEntity> {
        self.inner.one_chunk()
    }
}
