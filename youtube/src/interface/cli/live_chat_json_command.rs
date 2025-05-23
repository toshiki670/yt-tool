use crate::application::use_cases::chat_convert::ChatConvertUseCase;
use crate::infrastructure::io::chat_service_repository::IoChatServiceRepository;
use chrono::prelude::*;
use std::path::{Path, PathBuf};

/// This service provides an interface for managing and retrieving live chat JSON data from files.
pub struct LiveChatJsonCommand<'a, T> {
    source: &'a T,
}

impl<'a, T> LiveChatJsonCommand<'a, T> {
    /// Create a new LiveChatJsonService instance.
    ///
    /// # Arguments
    /// - `inner`: Source file path.
    pub fn new(source: &'a T) -> Self {
        Self { source }
    }
}

/// This implementation is for the PathBuf type.
impl LiveChatJsonCommand<'_, PathBuf> {
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

        let service = ChatConvertUseCase::new(repositories);

        service.convert_from_lines().await
    }

    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `file_type`: The file type to save the converted data.
    pub async fn generate_file_with_type(&self, file_type: &String) -> anyhow::Result<()> {
        let source_path = self.source.clone();
        let mut target_path = source_path.clone();
        target_path.set_extension(file_type);
        let target_path = target_path;

        let repositories = vec![IoChatServiceRepository::file_to_file(
            source_path,
            target_path,
        )?];

        let service = ChatConvertUseCase::new(repositories);

        service.convert_from_lines().await
    }
}

impl LiveChatJsonCommand<'_, Vec<PathBuf>> {
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

        let service = ChatConvertUseCase::new(repositories);

        service.convert_from_lines().await
    }

    /// Generate simple chat CSV data from live chat JSON data.
    pub async fn generate_files_with_timestamped_name(&self) -> anyhow::Result<()> {
        let source_paths = self.source.clone();

        let results = source_paths
            .into_iter()
            .map(|source_path| {
                let created_at = source_path.metadata()?.created()?;
                let created_at: DateTime<Local> = created_at.into();

                let target_path = source_path
                    .with_file_name(format!("{}.csv", created_at.format("%Y-%m-%dT%H-%M-%S%z")));

                let rp = IoChatServiceRepository::file_to_file(source_path, target_path)?;

                Ok(rp)
            })
            .collect::<Vec<_>>();

        let repositories = rust_support::anyhow::collect_results(results)?;

        let service = ChatConvertUseCase::new(repositories);

        service.convert_from_lines().await
    }
}

/// This implementation is for the String type.
impl LiveChatJsonCommand<'_, String> {
    /// Generate simple chat CSV data from live chat JSON data.
    ///
    /// # Arguments
    /// - `target_path`: The path to save the converted data.
    pub async fn generate_file_with_string(&self, target_path: &Path) -> anyhow::Result<()> {
        let source_string = self.source.clone();
        let target_path = target_path.to_path_buf();

        let repositories = vec![IoChatServiceRepository::in_memory_to_file(
            source_string,
            target_path,
        )?];

        let service = ChatConvertUseCase::new(repositories);

        service.convert_from_lines().await
    }
}
