use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::filesystem::chat_repository::FsChatRepository;
use std::path::PathBuf;

pub fn chat_convert(from_path: PathBuf, to_path: PathBuf) -> anyhow::Result<()> {
    let from_chat = Box::new(FsChatRepository::new(from_path)?);
    let to_chat = Box::new(FsChatRepository::new(to_path)?);
    let chat_convert_service = ChatConvertService::new(from_chat, to_chat);
    chat_convert_service.convert()
}
