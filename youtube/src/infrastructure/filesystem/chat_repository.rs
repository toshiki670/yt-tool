mod csv_struct;
mod json_struct;

use crate::domain::chat::ChatEntity;
use crate::domain::repositories::ChatRepository;
use anyhow::{bail, Context as _};
use std::{fs::File, path::PathBuf};

pub struct FsChatRepository {
    file: File,
    file_type: FileType,
}

impl FsChatRepository {
    pub fn new(file_path: &PathBuf) -> anyhow::Result<Self> {
        let file = File::open(file_path)
            .with_context(|| format!("Failed to open {}", &file_path.display()))?;
        let file_type = FileType::from_path(file_path)?;
        Ok(Self { file, file_type })
    }
}

impl ChatRepository for FsChatRepository {
    fn all(&self) -> anyhow::Result<Vec<ChatEntity>> {
        let chats;

        match self.file_type {
            FileType::Json => {
                chats = json_struct::JsonStruct::all_from_file(&self.file)
                    .context("Failed to read json file")?;
            }
            FileType::Csv => {
                unimplemented!()
            }
        }

        Ok(chats)
    }

    fn bulk_create(&self, chats: Vec<ChatEntity>) -> anyhow::Result<()> {
        match self.file_type {
            FileType::Json => {
                unimplemented!()
            }
            FileType::Csv => {
                csv_struct::CsvChat::bulk_create_file(chats, &self.file)
                    .context("Failed to create csv file")?;
            }
        }

        Ok(())
    }
}

enum FileType {
    Json,
    Csv,
}

impl FileType {
    fn from_path(path: &PathBuf) -> anyhow::Result<Self> {
        let extension = path
            .extension()
            .with_context(|| format!("No include extention in \"{}\"", &path.display()))?
            .to_str()
            .with_context(|| format!("Imvalid extention in \"{}\"", &path.display()))?;

        match extension {
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            _ => bail!("\"{}\" is unsupported extention", extension),
        }
    }
}
