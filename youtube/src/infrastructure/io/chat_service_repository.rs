use super::{
    live_chat_json_repository::IoLiveChatRepository,
    simple_chat_csv_repository::IoSimpleChatRepository,
};
use crate::domain::repositories::{
    ChatServiceRepository, FetchLiveChatRepository, SaveSimpleChatRepository,
};
use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::PathBuf,
};

/// This repository provides an interface for managing and retrieving live chat JSON data.
///
/// Internally, it uses a shared resource (`Rc<Mutex<T>>`) protected by a Mutex, enabling thread-safe operations.
/// It supports both reading live chat data from files and handling data in memory.
/// Additionally, it implements the FetchLiveChatRepository trait, supporting the retrieval of all live chat entities.
pub(crate) struct IoChatServiceRepository<R, W> {
    from_inner: IoLiveChatRepository<R>,
    to_inner: IoSimpleChatRepository<W>,
}

impl IoChatServiceRepository<File, File> {
    pub fn file_to_file(from_file_path: PathBuf, to_file_path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            from_inner: IoLiveChatRepository::build_opened_file(from_file_path)?,
            to_inner: IoSimpleChatRepository::build_created_file(to_file_path)?,
        })
    }
}

impl IoChatServiceRepository<File, Cursor<Vec<u8>>> {
    pub fn file_to_in_memory(from_file_path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            from_inner: IoLiveChatRepository::build_opened_file(from_file_path)?,
            to_inner: IoSimpleChatRepository::build_in_memory(Vec::new()),
        })
    }
}

impl IoChatServiceRepository<Cursor<String>, File> {
    pub fn in_memory_to_file(from_string: String, to_file_path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            from_inner: IoLiveChatRepository::build_in_memory(from_string),
            to_inner: IoSimpleChatRepository::build_created_file(to_file_path)?,
        })
    }
}

impl IoChatServiceRepository<Cursor<String>, Cursor<Vec<u8>>> {
    pub fn in_memory_to_in_memory(from_string: String) -> anyhow::Result<Self> {
        Ok(Self {
            from_inner: IoLiveChatRepository::build_in_memory(from_string),
            to_inner: IoSimpleChatRepository::build_in_memory(Vec::new()),
        })
    }
}



// impl<R, W> IoChatServiceRepository<R, W> {
//     pub fn from_source(&self) -> Option<PathBuf> {
//         self.from_inner.source.clone()
//     }
// }

impl<R> IoChatServiceRepository<R, Cursor<Vec<u8>>> {

    pub fn to_in_memory_data(&self) -> anyhow::Result<String> {
        let cursor_mutex = self.to_inner.clone_inner_mutex();
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
    fn from_chat_repository(&self) -> &dyn FetchLiveChatRepository {
        &self.from_inner
    }

    fn to_chat_repository(&self) -> &dyn SaveSimpleChatRepository {
        &self.to_inner
    }
}
