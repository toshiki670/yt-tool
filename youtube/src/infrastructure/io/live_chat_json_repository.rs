use crate::domain::{live_chat::LiveChatEntity, repositories::FetchLiveChatRepository};
use anyhow::Context as _;
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    path::PathBuf,
    rc::Rc,
    sync::Mutex,
};

/// This repository provides an interface for managing and retrieving live chat JSON data.
///
/// Internally, it uses a shared resource (`Rc<Mutex<T>>`) protected by a Mutex, enabling thread-safe operations.
/// It supports both reading live chat data from files and handling data in memory.
/// Additionally, it implements the FetchLiveChatRepository trait, supporting the retrieval of all live chat entities.
pub(crate) struct IoLiveChatRepository<T> {
    inner: Rc<Mutex<T>>,
    pub source: Option<PathBuf>,
}

/// `IoLiveChatRepository<File>` is a repository implemented based on files.
///
/// In this implementation, live chat JSON data is read from the specified file,
/// and `Rc<Mutex<File>>` is used to enable thread-safe access.
/// The `source` field is added to retain source information.
impl IoLiveChatRepository<File> {
    pub fn build_opened_file(file_path: PathBuf) -> anyhow::Result<Self> {
        let file = File::open(&file_path)
            .with_context(|| format!("Failed to open {}", file_path.display()))?;
        let file_mutex = Rc::new(Mutex::new(file));

        let repository = Self {
            inner: file_mutex,
            source: Some(file_path),
        };
        Ok(repository)
    }
}

/// `IoLiveChatRepository<Cursor<T>>` is a repository implemented based on in-memory data.
///
/// In this implementation, live chat JSON data is read from the specified in-memory data,
/// and `Rc<Mutex<Cursor<T>>>` is used to enable thread-safe access.
/// The `source` field is not retained.
impl<T> IoLiveChatRepository<Cursor<T>> {
    pub fn build_in_memory(inner: T) -> Self {
        let cursor = Cursor::new(inner);
        let cursor_mutex = Rc::new(Mutex::new(cursor));

        Self {
            inner: cursor_mutex,
            source: None,
        }
    }
}

/// `IoLiveChatRepository<R>` implements the `FetchLiveChatRepository` trait.
///
/// `R` is a type that implements the `Read` trait.
impl<R> FetchLiveChatRepository for IoLiveChatRepository<R>
where
    R: Read,
{
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>> {
        let inner_mutex = Rc::clone(&self.inner);
        let mut inner_lock = inner_mutex.lock().unwrap();
        let inner = &mut *inner_lock;
        let buffered = BufReader::new(inner);

        let mut live_chats = Vec::new();

        for (line_number, line) in buffered.lines().enumerate() {
            let line_number = line_number + 1;
            let line = line?;

            let live_chat = serde_json::from_str::<LiveChatEntity>(&line).with_context(|| {
                if let Some(source) = &self.source {
                    let source_str = source.to_string_lossy();
                    format!("Failed to convert at {source_str}:{line_number}")
                } else {
                    format!("Failed to convert at {line_number} row")
                }
            })?;

            live_chats.push(live_chat);
        }

        Ok(live_chats)
    }

    fn first(&self) -> anyhow::Result<LiveChatEntity> {
        let inner_mutex = Rc::clone(&self.inner);
        let mut inner_lock = inner_mutex.lock().unwrap();
        let inner = &mut *inner_lock;
        let mut buffered = BufReader::new(inner);

        let mut content = String::new();
        buffered.read_to_string(&mut content)?;

        let live_chat =
            serde_json::from_str::<LiveChatEntity>(&content).context("Failed to mapping a json")?;

        Ok(live_chat)
    }
}
