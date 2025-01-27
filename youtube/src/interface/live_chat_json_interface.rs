use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::io::chat_service_repository::IoChatServiceRepository;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// This service provides an interface for managing and retrieving live chat JSON data from files.
pub struct LiveChatJsonInterface<'a, T> {
    inner: &'a T,
}

impl<'a, T> LiveChatJsonInterface<'a, T> {
    /// Create a new LiveChatJsonService instance.
    ///
    /// # Arguments
    /// - `inner`: Source file path.
    pub fn new(inner: &'a T) -> Self {
        Self { inner }
    }
}

/// This implementation is for the PathBuf type.
impl LiveChatJsonInterface<'_, PathBuf> {
    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `to_path`: The path to save the converted data.
    pub async fn generate_file_with_path(&self, to_path: &Path) -> anyhow::Result<()> {
        let from_path = self.inner.clone();
        let to_path = to_path.to_path_buf();

        let repositories = vec![Arc::new(IoChatServiceRepository::file_to_file(
            from_path, to_path,
        )?)];

        let service = ChatConvertService::new(repositories);

        service.convert_from_lines().await
    }

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `file_type`: The file type to save the converted data.
    pub async fn generate_file_with_type(&self, file_type: &String) -> anyhow::Result<()> {
        let from_path = self.inner.clone();
        let mut to_path = from_path.clone();
        to_path.set_extension(file_type);
        let to_path = to_path;

        let repositories = vec![Arc::new(IoChatServiceRepository::file_to_file(
            from_path, to_path,
        )?)];

        let service = ChatConvertService::new(repositories);

        service.convert_from_lines().await
    }
}

impl LiveChatJsonInterface<'_, Vec<PathBuf>> {
    /// Generate simple chat CSV data from live chat JSON data.
    pub async fn generate_files_with_csv(&self) -> anyhow::Result<()> {
        let from_paths = self.inner.clone();

        let results = from_paths
            .into_iter()
            .map(|from_path| {
                let mut to_path = from_path.clone();
                to_path.set_extension("csv");
                let to_path = to_path;
                let rp = IoChatServiceRepository::file_to_file(from_path, to_path)?;

                Ok(Arc::new(rp))
            })
            .collect::<Vec<_>>();

        let repositories = support::anyhow::collect_results(results)?;

        let service = ChatConvertService::new(repositories);

        service.convert_from_lines().await
    }
}

/// This implementation is for the String type.
impl LiveChatJsonInterface<'_, String> {
    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `to_path`: The path to save the converted data.
    pub async fn generate_file_with_string(&self, to_path: &Path) -> anyhow::Result<()> {
        let from_string = self.inner.clone();
        let to_path = to_path.to_path_buf();

        let repositories = vec![Arc::new(IoChatServiceRepository::in_memory_to_file(
            from_string,
            to_path,
        )?)];

        let service = ChatConvertService::new(repositories);

        service.convert_from_lines().await
    }
}
