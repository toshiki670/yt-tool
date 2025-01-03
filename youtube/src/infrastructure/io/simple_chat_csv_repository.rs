use crate::domain::simple_chat::{repository::SaveSimpleChatRepository, SimpleChatEntity};
use anyhow::Context as _;
use std::{
    fs::File,
    io::{Cursor, Write},
    path::PathBuf,
    rc::Rc,
    sync::Mutex,
};

pub(crate) struct IoSimpleChatRepository<T> {
    inner: Rc<Mutex<T>>,
}

impl<T> IoSimpleChatRepository<T> {
    pub fn new(inner: &Rc<Mutex<T>>) -> Self {
        Self {
            inner: Rc::clone(inner),
        }
    }
}

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

impl<T> IoSimpleChatRepository<Cursor<T>> {
    pub fn build_in_memory(inner: T) -> anyhow::Result<(Rc<Mutex<Cursor<T>>>, Self)> {
        let cursor = Cursor::new(inner);
        let cursor_mutex = Rc::new(Mutex::new(cursor));

        let repository = Self {
            inner: Rc::clone(&cursor_mutex),
        };
        Ok((cursor_mutex, repository))
    }
}

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
