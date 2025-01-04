use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::io::{
    live_chat_json_repository::IoLiveChatRepository,
    simple_chat_csv_repository::IoSimpleChatRepository,
};

use std::path::PathBuf;

/// This service provides an interface for managing and retrieving live chat JSON data from files.
pub struct LiveChatJsonService<'a, T> {
    inner: &'a T,
}

impl<'a, T> LiveChatJsonService<'a, T> {
    /// Create a new LiveChatJsonService instance.
    ///
    /// # Arguments
    /// - `inner`: The inner path.
    ///
    /// # Returns
    /// - `Self`: LiveChatJsonService instance.
    pub fn new(inner: &'a T) -> Self {
        Self { inner }
    }
}

/// This implementation is for the PathBuf type.
impl<'a> LiveChatJsonService<'a, PathBuf> {
    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `to_path`: The path to save the converted data.
    ///
    /// # Returns
    /// - `anyhow::Result<()>`: Result of the conversion.
    pub fn generate_file_with_path(&self, to_path: &PathBuf) -> anyhow::Result<()> {
        let (_, live_chat_repository) = IoLiveChatRepository::build_opened_file(self.inner)?;
        let (_, simple_chat_repository) = IoSimpleChatRepository::build_created_file(to_path)?;

        let mut chat_convert_service =
            ChatConvertService::new(&live_chat_repository, &simple_chat_repository);

        chat_convert_service.convert_from_lines()
    }

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `file_type`: The file type to save the converted data.
    ///
    /// # Returns
    /// - `anyhow::Result<()>`: Result of the conversion.
    pub fn generate_file_with_type(&self, file_type: &String) -> anyhow::Result<()> {
        let mut to_path = self.inner.clone();
        to_path.set_extension(file_type);

        let (_, live_chat_repository) = IoLiveChatRepository::build_opened_file(self.inner)?;
        let (_, simple_chat_repository) = IoSimpleChatRepository::build_created_file(&to_path)?;

        let mut chat_convert_service =
            ChatConvertService::new(&live_chat_repository, &simple_chat_repository);

        chat_convert_service.convert_from_lines()
    }
}

/// This implementation is for the String type.
impl<'a> LiveChatJsonService<'a, String> {
    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `to_path`: The path to save the converted data.
    ///
    /// # Returns
    /// - `anyhow::Result<()>`: Result of the conversion.
    pub fn generate_file_with_path(&self, to_path: &PathBuf) -> anyhow::Result<()> {
        let (_, live_chat_repository) = IoLiveChatRepository::build_in_memory(self.inner);
        let (_, simple_chat_repository) = IoSimpleChatRepository::build_created_file(to_path)?;

        let mut chat_convert_service =
            ChatConvertService::new(&live_chat_repository, &simple_chat_repository);

        chat_convert_service.convert_from_lines()
    }
}
