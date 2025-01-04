use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::io::{
    live_chat_json_repository::IoLiveChatRepository,
    simple_chat_csv_repository::IoSimpleChatRepository,
};

/// This service provides an interface for managing and retrieving live chat JSON data in memory.
pub struct ChatInMemoryService;

impl ChatInMemoryService {

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `json_data`: The JSON data is not line-broken and consists of multiple lines.
    ///
    /// # Returns
    /// - `anyhow::Result<String>`: Result of the conversion.
    pub fn generate_from_lines(json_data: String) -> anyhow::Result<String> {
        let (_, live_chat_repository) = IoLiveChatRepository::build_in_memory(json_data.clone());
        let live_chat_repository = Box::new(live_chat_repository);

        let (cursor_mutex, simple_chat_repository) =
            IoSimpleChatRepository::build_in_memory(Vec::new());
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

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `json_chunk_data`: The JSON data is line-broken and consists one live chat JSON data.
    ///
    /// # Returns
    /// - `anyhow::Result<String>`: Result of the conversion.
    pub fn generate_from_chunk(json_chunk_data: String) -> anyhow::Result<String> {
        let (_, live_chat_repository) = IoLiveChatRepository::build_in_memory(json_chunk_data.clone());
        let live_chat_repository = Box::new(live_chat_repository);

        let (cursor_mutex, simple_chat_repository) =
            IoSimpleChatRepository::build_in_memory(Vec::new());
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
