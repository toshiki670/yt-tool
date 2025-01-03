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

    pub fn convert_with_path(&self, to_path: &PathBuf) -> anyhow::Result<()> {
        let (_, live_chat_repository) = IoLiveChatRepository::build_opened_file(self.base_path)?;
        let live_chat_repository = Box::new(live_chat_repository);

        let (_, simple_chat_repository) = IoSimpleChatRepository::build_created_file(to_path)?;
        let simple_chat_repository = Box::new(simple_chat_repository);

        let mut chat_convert_service =
            ChatConvertService::new(live_chat_repository, simple_chat_repository);
        chat_convert_service.convert_with_lines()
    }

    pub fn convert_with_type(&self, file_type: &String) -> anyhow::Result<()> {
        let mut to_path = self.base_path.clone();
        to_path.set_extension(file_type);

        let (_, live_chat_repository) = IoLiveChatRepository::build_opened_file(self.base_path)?;
        let live_chat_repository = Box::new(live_chat_repository);

        let (_, simple_chat_repository) = IoSimpleChatRepository::build_created_file(&to_path)?;
        let simple_chat_repository = Box::new(simple_chat_repository);

        let mut chat_convert_service =
            ChatConvertService::new(live_chat_repository, simple_chat_repository);
        chat_convert_service.convert_with_lines()
    }
}
