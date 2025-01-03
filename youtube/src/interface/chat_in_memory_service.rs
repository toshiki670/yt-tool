use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::io::{
    live_chat_json_repository::IoLiveChatRepository,
    simple_chat_csv_repository::IoSimpleChatRepository,
};

pub struct ChatInMemoryService<'a> {
    value: &'a String,
}

impl<'a> ChatInMemoryService<'a> {
    pub fn new(value: &'a String) -> Self {
        Self { value }
    }

    pub fn convert_as_lines(&self) -> anyhow::Result<String> {
        let (_, live_chat_repository) = IoLiveChatRepository::build_in_memory(self.value.clone())?;
        let live_chat_repository = Box::new(live_chat_repository);

        let (cursor_mutex, simple_chat_repository) =
            IoSimpleChatRepository::build_in_memory(Vec::new())?;
        let simple_chat_repository = Box::new(simple_chat_repository);

        let mut chat_convert_service =
            ChatConvertService::new(live_chat_repository, simple_chat_repository);
        chat_convert_service.convert_with_lines()?;

        let mut cursor_lock = cursor_mutex.lock().unwrap();
        let cursor = &mut *cursor_lock;

        let data = cursor.get_ref();
        let data_str = String::from_utf8(data.to_vec())?;
        Ok(data_str)
    }

    pub fn convert_as_chunk(&self) -> anyhow::Result<String> {
        let (_, live_chat_repository) = IoLiveChatRepository::build_in_memory(self.value.clone())?;
        let live_chat_repository = Box::new(live_chat_repository);

        let (cursor_mutex, simple_chat_repository) =
            IoSimpleChatRepository::build_in_memory(Vec::new())?;
        let simple_chat_repository = Box::new(simple_chat_repository);

        let mut chat_convert_service =
            ChatConvertService::new(live_chat_repository, simple_chat_repository);
        chat_convert_service.convert_with_one_chunk()?;

        let mut cursor_lock = cursor_mutex.lock().unwrap();
        let cursor = &mut *cursor_lock;

        let data = cursor.get_ref();
        let data_str = String::from_utf8(data.to_vec())?;
        Ok(data_str)
    }
}
