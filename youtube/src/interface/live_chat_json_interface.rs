use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::io::chat_service_repository::IoChatServiceRepository;
use std::path::PathBuf;
use std::sync::Arc;

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
    pub async fn generate_file_with_path(&self, to_path: &PathBuf) -> anyhow::Result<()> {
        let from_path = self.inner.clone();
        let to_path = to_path.clone();

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
    ///
    /// # Returns
    /// - `anyhow::Result<()>`: Result of the conversion.
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

impl<'a> LiveChatJsonService<'a, Vec<PathBuf>> {
    /// 複数のパスに対して簡単なチャットCSVデータを生成します。
    ///
    /// # 引数
    /// - `to_paths`: 変換されたデータを保存するパスの配列。
    ///
    /// # 戻り値
    /// - `anyhow::Result<()>`: 変換の結果。
    pub async fn generate_files_with_csv(&self) -> anyhow::Result<()> {
        let from_paths = self.inner.clone();

        let repositories = from_paths
            .into_iter()
            .map(|from_path| {
                let mut to_path = from_path.clone();
                to_path.set_extension("csv");
                let to_path = to_path;
                let rp = IoChatServiceRepository::file_to_file(from_path, to_path)?;

                Ok(Arc::new(rp))
            })
            .collect::<Vec<_>>();

        let (oks, errs): (Vec<_>, Vec<_>) = repositories.into_iter().partition(Result::is_ok);
        let repositories: Vec<_> = oks.into_iter().map(Result::unwrap).collect();
        let errors: Vec<anyhow::Error> = errs.into_iter().filter_map(Result::err).collect();

        // If there are errors, return a concated error
        if !errors.is_empty() {
            let combined_error = errors
                .into_iter()
                .fold(anyhow::anyhow!("Multiple errors occurred"), |acc, e| {
                    acc.context(e)
                });

            anyhow::bail!(combined_error);
        }

        let service = ChatConvertService::new(repositories);

        service.convert_from_lines().await
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
    pub async fn generate_file_with_string(&self, to_path: &PathBuf) -> anyhow::Result<()> {
        let from_string = self.inner.clone();
        let to_path = to_path.clone();

        let repositories = vec![Arc::new(IoChatServiceRepository::in_memory_to_file(
            from_string,
            to_path,
        )?)];

        let service = ChatConvertService::new(repositories);

        service.convert_from_lines().await
    }
}
