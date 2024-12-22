use std::path::PathBuf;

mod csv_struct;
mod json_struct;

use super::super::super::domain::repositories::ChatRepository as ChatRepositoryTrait;

struct ChatRepository {
    file_path: PathBuf,
}

impl ChatRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}

impl ChatRepositoryTrait for ChatRepository {
    fn all(&self) -> Vec<super::super::super::domain::chat::Chat> {
        unimplemented!()
    }

    fn bulk_create(&self, chats: Vec<super::super::super::domain::chat::Chat>) {
        unimplemented!()
    }
}
