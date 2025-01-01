use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::filesystem::chat_repository::FsChatRepository;
use std::path::PathBuf;

pub struct ChatFileService<'a> {
    base_path: &'a PathBuf,
}

impl<'a> ChatFileService<'a> {
    pub fn new(base_path: &'a PathBuf) -> Self {
        Self { base_path }
    }

    pub fn convert_with_path(&self, path: &PathBuf) -> anyhow::Result<()> {
        let from_chat = Box::new(FsChatRepository::open(self.base_path)?);
        let to_chat = Box::new(FsChatRepository::create(path)?);
        let chat_convert_service = ChatConvertService::new(from_chat, to_chat);
        chat_convert_service.convert()
    }

    pub fn convert_with_type(&self, file_type: &String) -> anyhow::Result<()> {
        let mut to_path = self.base_path.clone();
        to_path.set_extension(file_type);

        let from_chat = Box::new(FsChatRepository::open(self.base_path)?);
        let to_chat = Box::new(FsChatRepository::create(&to_path)?);
        let chat_convert_service = ChatConvertService::new(from_chat, to_chat);
        chat_convert_service.convert()
    }
}
