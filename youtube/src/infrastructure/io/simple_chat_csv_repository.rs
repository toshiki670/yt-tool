use crate::domain::simple_chat::{repository::SaveSimpleChatRepository, SimpleChatEntity};
use anyhow::Context as _;
use std::{
    fs::File,
    io::{Cursor, Write},
    path::PathBuf,
    rc::Rc,
    sync::Mutex,
};

/// This repository provides an interface for managing and retrieving simple chat CSV data.
///
/// Internally, it uses a shared resource (`Rc<Mutex<T>>`) protected by a Mutex, enabling thread-safe operations.
/// It supports both writing simple chat data to files and handling data in memory.
/// Additionally, it implements the SaveSimpleChatRepository trait, supporting the bulk creation of simple chat entities.
pub(crate) struct IoSimpleChatRepository<T> {
    inner: Rc<Mutex<T>>,
}

/// `IoSimpleChatRepository<File>` is a repository implemented based on files.
///
/// In this implementation, simple chat CSV data is written to the specified file,
/// and `Rc<Mutex<File>>` is used to enable thread-safe access.
impl IoSimpleChatRepository<File> {
    pub fn build_created_file(file_path: &PathBuf) -> anyhow::Result<(Rc<Mutex<File>>, Self)> {
        let file = File::create(file_path).context("Failed to create file")?;
        let file_mutex = Rc::new(Mutex::new(file));

        let repository = Self {
            inner: Rc::clone(&file_mutex),
        };
        Ok((file_mutex, repository))
    }
}

/// `IoSimpleChatRepository<Cursor<T>>` is a repository implemented based on in-memory data.
///
/// In this implementation, simple chat CSV data is written to the specified in-memory data,
/// and `Rc<Mutex<Cursor<T>>>` is used to enable thread-safe access.
impl<T> IoSimpleChatRepository<Cursor<T>> {
    pub fn build_in_memory(inner: T) -> (Rc<Mutex<Cursor<T>>>, Self) {
        let cursor = Cursor::new(inner);
        let cursor_mutex = Rc::new(Mutex::new(cursor));

        let repository = Self {
            inner: Rc::clone(&cursor_mutex),
        };
        (cursor_mutex, repository)
    }
}

/// `IoSimpleChatRepository<W>` implements the `SaveSimpleChatRepository` trait.
///
/// `W` is a type that implements the `Write` trait.
impl<W> SaveSimpleChatRepository for IoSimpleChatRepository<W>
where
    W: Write,
{
    fn bulk_create(&self, simple_chats: Vec<SimpleChatEntity>) -> anyhow::Result<()> {
        let inner_mutex = Rc::clone(&self.inner);
        let mut inner_lock = inner_mutex.lock().unwrap();
        let inner = &mut *inner_lock;

        let mut wtr = csv::Writer::from_writer(inner);

        for simple_chat in simple_chats {
            wtr.serialize(&simple_chat)
                .with_context(|| format!("Failed to serialize at {:?}", &simple_chat))?;
        }
        wtr.flush().context("Failed to flush")?;

        Ok(())
    }
}
