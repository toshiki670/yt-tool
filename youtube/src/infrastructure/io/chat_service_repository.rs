use super::{
    live_chat_json_repository::IoLiveChatRepository,
    simple_chat_csv_repository::IoSimpleChatRepository,
};
use crate::domain::{
    repositories::{ChatServiceRepository, FetchLiveChatRepository, SaveSimpleChatRepository},
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};
use anyhow::Context as _;
use chrono::prelude::*;
use futures::future;
use log::{error, info};
use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::PathBuf,
};
use support::anyhow::collect_results;
use tokio::time::Instant;

/// This repository provides an interface for managing and retrieving live chat JSON data.
///
/// Internally, it uses a shared resource (`Rc<Mutex<T>>`) protected by a Mutex, enabling thread-safe operations.
/// It supports both reading live chat data from files and handling data in memory.
/// Additionally, it implements the FetchLiveChatRepository trait, supporting the retrieval of all live chat entities.
pub(crate) struct IoChatServiceRepository<R, W> {
    source: IoLiveChatRepository<R>,
    target: IoSimpleChatRepository<W>,
}

impl IoChatServiceRepository<File, File> {
    pub fn file_to_file(from_file_path: PathBuf, to_file_path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            source: IoLiveChatRepository::build_opened_file(from_file_path)?,
            target: IoSimpleChatRepository::build_created_file(to_file_path)?,
        })
    }
}

impl IoChatServiceRepository<File, Cursor<Vec<u8>>> {
    pub fn file_to_in_memory(from_file_path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            source: IoLiveChatRepository::build_opened_file(from_file_path)?,
            target: IoSimpleChatRepository::build_in_memory(Vec::new()),
        })
    }
}

impl IoChatServiceRepository<Cursor<String>, File> {
    pub fn in_memory_to_file(from_string: String, to_file_path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            source: IoLiveChatRepository::build_in_memory(from_string),
            target: IoSimpleChatRepository::build_created_file(to_file_path)?,
        })
    }
}

impl IoChatServiceRepository<Cursor<String>, Cursor<Vec<u8>>> {
    pub fn in_memory_to_in_memory(from_string: String) -> anyhow::Result<Self> {
        Ok(Self {
            source: IoLiveChatRepository::build_in_memory(from_string),
            target: IoSimpleChatRepository::build_in_memory(Vec::new()),
        })
    }
}

impl<R, W> IoChatServiceRepository<R, W> {
    pub fn source_path(&self) -> Option<PathBuf> {
        self.source.path.clone()
    }
}

impl<R> IoChatServiceRepository<R, Cursor<Vec<u8>>> {
    pub fn to_in_memory_data(&self) -> anyhow::Result<String> {
        let cursor_mutex = self.target.clone_inner_mutex();
        let mut cursor_lock = cursor_mutex.lock().unwrap();
        let cursor = &mut *cursor_lock;
        let data = cursor.get_ref();
        let data_str = String::from_utf8(data.to_vec())?;
        Ok(data_str)
    }
}

impl<R, W> ChatServiceRepository for IoChatServiceRepository<R, W>
where
    R: Read,
    W: Write,
{
    async fn convert_from_lines(&self) -> anyhow::Result<()> {
        let start = Instant::now();

        let from_chats = self.source.all()?;

        let futures = from_chats
            .into_iter()
            .map(|f| f.try_into_simple_chats())
            .collect::<Vec<_>>();

        let results = future::join_all(futures)
            .await
            .into_iter()
            .enumerate()
            .map(|(n, f)| {
                if let Some(source) = self.source_path() {
                    f.with_context(|| {
                        format!(
                            "Failed to convert live chat at {}:{}",
                            source.display(),
                            n + 1,
                        )
                    })
                } else {
                    f.with_context(|| format!("Failed to convert live chat at line {}", n + 1))
                }
            })
            .collect::<Vec<_>>();

        let init_simple_chat = if let Some(simple_chat) = self.create_metadata_simple_chat() {
            vec![simple_chat]
        } else {
            vec![]
        };

        let simple_chats =
            collect_results(results)?
                .into_iter()
                .fold(init_simple_chat, |mut acc, chats| {
                    acc.extend(chats);
                    acc
                });

        self.target.bulk_create(simple_chats)?;

        let duration = start.elapsed();
        if let Some(source) = self.source_path() {
            info!(
                "Conversion time was {:.3?} from this {}",
                duration,
                source.display()
            );
        } else {
            info!("Conversion time was {:.3?}", duration);
        }
        Ok(())
    }

    async fn convert_from_chunk(&self) -> anyhow::Result<()> {
        let error_context = || {
            if let Some(source) = self.source_path() {
                format!("Failed to convert live chat from {}", source.display())
            } else {
                "Failed to convert live chat".to_string()
            }
        };

        let live_chat = self.source.first().with_context(error_context)?;
        let simple_chat = live_chat.try_into().with_context(error_context)?;
        self.target
            .bulk_create(simple_chat)
            .with_context(error_context)?;

        Ok(())
    }
}

impl<R, W> IoChatServiceRepository<R, W> {
    fn create_metadata_simple_chat(&self) -> Option<SimpleChatEntity> {
        if let Some(source) = self.source_path() {
            let mut content = Content::new();

            let metadata = match source.metadata() {
                Ok(m) => m,
                Err(e) => {
                    error!("Failed to get metadata from {}: {}", source.display(), e);
                    return None;
                }
            };

            let created_at = match metadata.created() {
                Ok(m) => m,
                Err(e) => {
                    error!("Failed to get created at from {}: {}", source.display(), e);
                    return None;
                }
            };

            let created_at: DateTime<Local> = created_at.into();
            let posted_at = Some(created_at.into());

            if let Some(file_name) = source.file_name() {
                content.add("fileName", Some(file_name.to_string_lossy().to_string()));
            }

            Some(SimpleChatEntity {
                id: "".to_string(),
                posted_at,
                author_external_channel_id: None,
                category: CategoryValue::Metadata,
                author_name: None,
                content,
            })
        } else {
            None
        }
    }
}
