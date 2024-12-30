use std::{
    fs::File,
    io::{BufRead as _, BufReader},
    path::PathBuf,
};

mod csv_struct;
mod json_struct;

use anyhow::Context;

use crate::domain::chat::Chat;

use super::super::super::domain::repositories::ChatRepository;

pub struct FsChatRepository {
    file: File,
    file_type: FileType,
}

impl FsChatRepository {
    pub fn new(file_path: &PathBuf) -> anyhow::Result<Self> {
        let file = File::open(file_path)?;
        let file_type = FileType::from_path(file_path)?;
        Ok(Self { file, file_type })
    }
}

impl ChatRepository for FsChatRepository {
    fn all(&self) -> anyhow::Result<Vec<Chat>> {
        let chats;

        match self.file_type {
            FileType::Json => {
                chats = json_struct::JsonChat::all_from_file(&self.file)?;
            }
            FileType::Csv => {
                unimplemented!()
            }
        }

        Ok(chats)
    }

    fn bulk_create(&self, chats: Vec<Chat>) -> anyhow::Result<()> {
        match self.file_type {
            FileType::Json => {
                unimplemented!()
            }
            FileType::Csv => {
                csv_struct::CsvChat::bulk_create_file(chats, &self.file)?;
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
            .context(FsChatRepositoryError::NoExtensionError(path.clone()))?
            .to_str()
            .context(FsChatRepositoryError::InvalidExtensionError(path.clone()))?;

        match extension {
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            _ => Err(FsChatRepositoryError::UnsupportedExtensionError.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FsChatRepositoryError {
    #[error("\"{}\" has no extension", .0.display())]
    NoExtensionError(PathBuf),

    #[error("\"{}\" has an invalid extension", .0.display())]
    InvalidExtensionError(PathBuf),

    #[error("Unsupported extension")]
    UnsupportedExtensionError,
}
