use crate::application::chat_service::ChatConvertService;
use crate::infrastructure::io::live_chat_json_repository::file::FileLiveChatRepository;
use crate::infrastructure::io::simple_chat_csv_repository::file::FileSimpleChatRepository;
use std::path::PathBuf;

pub struct ChatFileService<'a> {
    base_path: &'a PathBuf,
}

impl<'a> ChatFileService<'a> {
    pub fn new(base_path: &'a PathBuf) -> Self {
        Self { base_path }
    }

    pub fn convert_with_path(&self, path: &PathBuf) -> anyhow::Result<()> {
        let live_chat_repository = Box::new(FileLiveChatRepository::open(self.base_path)?);
        let simple_chat_repository = Box::new(FileSimpleChatRepository::create(path)?);
        let chat_convert_service =
            ChatConvertService::new(live_chat_repository, simple_chat_repository);
        chat_convert_service.convert()
    }

    pub fn convert_with_type(&self, file_type: &String) -> anyhow::Result<()> {
        let mut to_path = self.base_path.clone();
        to_path.set_extension(file_type);

        let live_chat_repository = Box::new(FileLiveChatRepository::open(self.base_path)?);
        let simple_chat_repository = Box::new(FileSimpleChatRepository::create(&to_path)?);
        let chat_convert_service =
            ChatConvertService::new(live_chat_repository, simple_chat_repository);
        chat_convert_service.convert()
    }
}
