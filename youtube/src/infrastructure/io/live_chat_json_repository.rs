use crate::domain::live_chat::{repository::FetchLiveChatRepository, LiveChatEntity};
use anyhow::Context as _;
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    path::PathBuf,
    rc::Rc,
    sync::Mutex,
};

pub(crate) struct IoLiveChatRepository<T> {
    inner: Rc<Mutex<T>>,
}

impl IoLiveChatRepository<File> {
    pub fn build_opened_file(file_path: &PathBuf) -> anyhow::Result<(Rc<Mutex<File>>, Self)> {
        let file = File::open(file_path).context("Failed to create file")?;
        let file_mutex = Rc::new(Mutex::new(file));

        let repository = Self {
            inner: Rc::clone(&file_mutex),
        };
        Ok((file_mutex, repository))
    }
}

impl<T> IoLiveChatRepository<Cursor<T>> {
    pub fn build_in_memory(inner: T) -> anyhow::Result<(Rc<Mutex<Cursor<T>>>, Self)> {
        let cursor = Cursor::new(inner);
        let cursor_mutex = Rc::new(Mutex::new(cursor));

        let repository = Self {
            inner: Rc::clone(&cursor_mutex),
        };
        Ok((cursor_mutex, repository))
    }
}

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

            let live_chat = serde_json::from_str::<LiveChatEntity>(&line)
                .with_context(|| format!("Failed to convert at {line_number} row"))?;

            live_chats.push(live_chat);
        }

        Ok(live_chats)
    }

    fn one_chunk(&self) -> anyhow::Result<LiveChatEntity> {
        let inner_mutex = Rc::clone(&self.inner);
        let mut inner_lock = inner_mutex.lock().unwrap();
        let inner = &mut *inner_lock;
        let mut buffered = BufReader::new(inner);

        let mut content = String::new();
        buffered.read_to_string(&mut content)?;

        let live_chat = serde_json::from_str::<LiveChatEntity>(&content)
            .with_context(|| format!("Failed to convert the content"))?;

        Ok(live_chat)
    }
}
