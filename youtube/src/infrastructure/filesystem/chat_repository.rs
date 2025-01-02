use crate::domain::{
    live_chat::{repository::FetchLiveChatRepository, LiveChatEntity},
    simple_chat::{repository::SaveSimpleChatRepository, SimpleChatEntity},
};
use anyhow::{bail, Context as _};
use std::io::{BufRead as _, BufReader, Read as _};
use std::{fs::File, path::PathBuf};

pub struct FsChatRepository {
    file: File,
    file_path: PathBuf,
    file_type: FileType,
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

impl FsChatRepository {
    pub fn create(file_path: &PathBuf) -> anyhow::Result<Self> {
        let file = File::create(file_path)
            .with_context(|| format!("Failed to create {}", &file_path.display()))?;

        let file_type = FileType::from_path(file_path)?;
        Ok(Self {
            file,
            file_type,
            file_path: file_path.clone(),
        })
    }

    pub fn open(file_path: &PathBuf) -> anyhow::Result<Self> {
        let file = File::open(file_path)
            .with_context(|| format!("Failed to open {}", &file_path.display()))?;

        let file_type = FileType::from_path(file_path)?;
        Ok(Self {
            file,
            file_type,
            file_path: file_path.clone(),
        })
    }
}

impl FetchLiveChatRepository for FsChatRepository {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>> {
        let mut live_chats = Vec::new();

        for (line_number, line) in BufReader::new(&self.file).lines().enumerate() {
            let line_number = line_number + 1;
            let line = line?;

            let live_chat = serde_json::from_str::<LiveChatEntity>(&line).with_context(|| {
                format!(
                    "Failed to convert at {}:{}",
                    &self.file_path.display(),
                    line_number
                )
            })?;

            live_chats.push(live_chat);
        }

        Ok(live_chats)
    }

    fn one_chunk(&self) -> anyhow::Result<LiveChatEntity> {
        let mut content = String::new();
        BufReader::new(&self.file).read_to_string(&mut content)?;

        let live_chat = serde_json::from_str::<LiveChatEntity>(&content).with_context(|| {
            format!(
                "Failed to convert the content of the file: {}",
                &self.file_path.display()
            )
        })?;

        Ok(live_chat)
    }
}

impl SaveSimpleChatRepository for FsChatRepository {
    fn bulk_create(&self, simple_chats: Vec<SimpleChatEntity>) -> anyhow::Result<()> {
        match self.file_type {
            FileType::Json => {
                unimplemented!()
            }
            FileType::Csv => {
                let mut wtr = csv::Writer::from_writer(&self.file);

                for simple_chat in simple_chats {
                    wtr.serialize(&simple_chat)
                        .with_context(|| format!("Failed to serialize at {:?}", &simple_chat))?;
                }
                wtr.flush().context("Failed to flush")?;
            }
        }

        Ok(())
    }
}
