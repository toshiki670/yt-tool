use crate::domain::chat::ChatEntity;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CsvChat {
    pub timestamp_usec: String,
    pub author_external_channel_id: String,
    pub author_name: String,
    pub message: String,
    pub is_moderator: bool,
    pub membership_months: String,
    pub category: String,
}

impl CsvChat {
    pub fn bulk_create_file(chats: Vec<ChatEntity>, file: &File) -> anyhow::Result<()> {
        let mut wtr = csv::Writer::from_writer(file);

        for chat in chats {
            let chat: CsvChat = chat.into();
            wtr.serialize(chat)?;
        }

        wtr.flush()?;
        Ok(())
    }
}

impl<'a> From<ChatEntity> for CsvChat {
    fn from(chat: ChatEntity) -> Self {
        Self {
            timestamp_usec: chat.posted_at.to_string(),
            author_external_channel_id: chat.author_external_channel_id,
            author_name: chat.author_name,
            message: chat.message,
            is_moderator: chat.is_moderator,
            membership_months: chat.membership_months,
            category: chat.category.to_string(),
        }
    }
}
