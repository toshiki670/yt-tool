use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::io::{
    live_chat_json_repository::IoLiveChatRepository,
    simple_chat_csv_repository::IoSimpleChatRepository,
};

use std::path::PathBuf;

pub struct ChatFileService<'a> {
    base_path: &'a PathBuf,
}

impl<'a> ChatFileService<'a> {
    pub fn new(base_path: &'a PathBuf) -> Self {
        Self { base_path }
    }

    pub fn convert_with_path(&self, path: &PathBuf) -> anyhow::Result<()> {
        let live_chat_repository = Box::new(IoLiveChatRepository::from(self.base_path.clone()));
        let simple_chat_repository = Box::new(IoSimpleChatRepository::from(path.clone()));
        let mut chat_convert_service =
            ChatConvertService::new(live_chat_repository, simple_chat_repository);
        chat_convert_service.convert()
    }

    pub fn convert_with_type(&self, file_type: &String) -> anyhow::Result<()> {
        let mut to_path = self.base_path.clone();
        to_path.set_extension(file_type);

        let live_chat_repository = Box::new(IoLiveChatRepository::from(self.base_path.clone()));
        let simple_chat_repository = Box::new(IoSimpleChatRepository::from(to_path.clone()));
        let mut chat_convert_service =
            ChatConvertService::new(live_chat_repository, simple_chat_repository);
        chat_convert_service.convert()
    }
}
