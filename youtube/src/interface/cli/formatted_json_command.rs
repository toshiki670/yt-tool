use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::io::chat_service_repository::IoChatServiceRepository;
use std::path::{Path, PathBuf};

/// This service provides an interface for managing and retrieving live chat JSON data from files.
pub struct FormattedJsonCommand<'a, T> {
    source: &'a T,
}

impl<'a, T> FormattedJsonCommand<'a, T> {
    /// Create a new FormatedJsonService instance.
    ///
    /// # Arguments
    /// - `inner`: Source file path.
    pub fn new(source: &'a T) -> Self {
        Self { source }
    }
}

/// This implementation is for the PathBuf type.
impl FormattedJsonCommand<'_, PathBuf> {
    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `target_path`: The path to save the converted data.
    pub async fn generate_file_with_path(&self, target_path: &Path) -> anyhow::Result<()> {
        let source_path = self.source.clone();
        let target_path = target_path.to_path_buf();

        let repositories = vec![IoChatServiceRepository::file_to_file(
            source_path,
            target_path,
        )?];

        let service = ChatConvertService::new(repositories);

        service.convert_from_chunk().await
    }

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `file_type`: The file type to save the converted data.
    pub async fn generate_file_with_type(&self, file_type: &String) -> anyhow::Result<()> {
        let source_path = self.source.clone();
        let mut target_path = source_path.clone();
        target_path.set_extension(file_type);

        let repositories = vec![IoChatServiceRepository::file_to_file(
            source_path,
            target_path,
        )?];

        let service = ChatConvertService::new(repositories);
        service.convert_from_chunk().await
    }

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Returns
    /// - `anyhow::Result<String>`: Result of the conversion.
    pub async fn generate_string(&self) -> anyhow::Result<String> {
        let source_path = self.source.clone();

        let repositories = vec![IoChatServiceRepository::file_to_in_memory(source_path)?];

        let service = ChatConvertService::new(repositories);
        service.convert_from_chunk().await?;

        let repositories = service.move_chat_service_repository();

        let data_str = repositories.first().unwrap().to_in_memory_data()?;
        Ok(data_str)
    }
}

impl FormattedJsonCommand<'_, Vec<PathBuf>> {
    /// Generate simple chat CSV data from live chat JSON data.
    pub async fn generate_files_with_csv(&self) -> anyhow::Result<()> {
        let source_paths = self.source.clone();

        let results = source_paths
            .into_iter()
            .map(|source_path| {
                let mut target_path = source_path.clone();
                target_path.set_extension("csv");
                let target_path = target_path;
                let rp = IoChatServiceRepository::file_to_file(source_path, target_path)?;

                Ok(rp)
            })
            .collect::<Vec<_>>();

        let repositories = rust_support::anyhow::collect_results(results)?;

        let service = ChatConvertService::new(repositories);

        service.convert_from_chunk().await
    }
}

/// This implementation is for the String type.
impl FormattedJsonCommand<'_, String> {
    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `to_path`: The path to save the converted data.
    pub async fn generate_file_with_path(&self, target_path: &Path) -> anyhow::Result<()> {
        let source_string = self.source.clone();
        let target_path = target_path.to_path_buf();

        let repositories = vec![IoChatServiceRepository::in_memory_to_file(
            source_string,
            target_path,
        )?];

        let service = ChatConvertService::new(repositories);

        service.convert_from_chunk().await
    }

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Returns
    /// - `anyhow::Result<String>`: Result of the conversion.
    pub async fn generate_string(&self) -> anyhow::Result<String> {
        let source_string = self.source.clone();

        let repositories = vec![IoChatServiceRepository::in_memory_to_in_memory(
            source_string,
        )?];

        let service = ChatConvertService::new(repositories);
        service.convert_from_chunk().await?;

        let repositories = service.move_chat_service_repository();

        let data_str = repositories.first().unwrap().to_in_memory_data()?;
        Ok(data_str)
    }
}
