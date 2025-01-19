use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::io::chat_service_repository::IoChatServiceRepository;
use std::path::PathBuf;
use std::sync::Arc;

/// This service provides an interface for managing and retrieving live chat JSON data from files.
pub struct FormattedJsonInterface<'a, T> {
    inner: &'a T,
}

impl<'a, T> FormattedJsonInterface<'a, T> {
    /// Create a new FormatedJsonService instance.
    ///
    /// # Arguments
    /// - `inner`: The inner path.
    ///
    /// # Returns
    /// - `Self`: FormatedJsonService instance.
    pub fn new(inner: &'a T) -> Self {
        Self { inner }
    }
}

/// This implementation is for the PathBuf type.
impl<'a> FormattedJsonInterface<'a, PathBuf> {
    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `to_path`: The path to save the converted data.
    ///
    /// # Returns
    /// - `anyhow::Result<()>`: Result of the conversion.
    pub async fn generate_file_with_path(&self, to_path: &PathBuf) -> anyhow::Result<()> {
        let from_path = self.inner.clone();
        let to_path = to_path.clone();

        let repositories = vec![Arc::new(IoChatServiceRepository::file_to_file(
            from_path, to_path,
        )?)];

        let service = ChatConvertService::new(repositories);

        service.convert_from_chunk().await
    }

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `file_type`: The file type to save the converted data.
    ///
    /// # Returns
    /// - `anyhow::Result<()>`: Result of the conversion.
    pub async fn generate_file_with_type(&self, file_type: &String) -> anyhow::Result<()> {
        let from_path = self.inner.clone();
        let mut to_path = from_path.clone();
        to_path.set_extension(file_type);

        let repositories = vec![Arc::new(IoChatServiceRepository::file_to_file(
            from_path, to_path,
        )?)];

        let service = ChatConvertService::new(repositories);
        service.convert_from_chunk().await
    }

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Returns
    /// - `anyhow::Result<String>`: Result of the conversion.
    pub async fn generate_string(&self) -> anyhow::Result<String> {
        let from_path = self.inner.clone();

        let repositories = vec![Arc::new(IoChatServiceRepository::file_to_in_memory(
            from_path,
        )?)];

        let service = ChatConvertService::new(repositories);
        service.convert_from_chunk().await?;

        let repositories = service.move_chat_service_repository();

        let data_str = repositories.first().unwrap().to_in_memory_data()?;
        Ok(data_str)
    }
}

/// This implementation is for the String type.
impl<'a> FormattedJsonInterface<'a, String> {
    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `to_path`: The path to save the converted data.
    ///
    /// # Returns
    /// - `anyhow::Result<()>`: Result of the conversion.
    pub async fn generate_file_with_path(&self, to_path: &PathBuf) -> anyhow::Result<()> {
        let from_string = self.inner.clone();
        let to_path = to_path.clone();

        let repositories = vec![Arc::new(IoChatServiceRepository::in_memory_to_file(
            from_string,
            to_path,
        )?)];

        let service = ChatConvertService::new(repositories);

        service.convert_from_chunk().await
    }

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Returns
    /// - `anyhow::Result<String>`: Result of the conversion.
    pub async fn generate_string(&self) -> anyhow::Result<String> {
        let from_string = self.inner.clone();

        let repositories = vec![Arc::new(IoChatServiceRepository::in_memory_to_in_memory(
            from_string,
        )?)];

        let service = ChatConvertService::new(repositories);
        service.convert_from_chunk().await?;

        let repositories = service.move_chat_service_repository();

        let data_str = repositories.first().unwrap().to_in_memory_data()?;
        Ok(data_str)
    }
}
