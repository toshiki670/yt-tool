use crate::domain::chat::{Category, Chat as ChatDomain};
use serde::{Deserialize, Serialize};
use std::{fmt, fs::File};

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::ChatTextMessage => write!(f, "Chat text message"),
            Category::ChatPaidMessage => write!(f, "Chat paid message"),
            Category::ChatSponsorshipsGiftRedemptionAnnouncement => {
                write!(f, "Chat sponsorships gift redemption announcement")
            }
            Category::ChatTickerPaidMessageItem => write!(f, "Chat ticker paid message item"),
        }
    }
}

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
    pub fn bulk_create_file(chats: Vec<ChatDomain>, file: &File) -> anyhow::Result<()> {
        let mut wtr = csv::Writer::from_writer(file);

        for chat in chats {
            let chat: CsvChat = chat.into();
            wtr.serialize(chat)?;
        }

        wtr.flush()?;
        Ok(())
    }
}

impl<'a> From<ChatDomain> for CsvChat {
    fn from(chat: ChatDomain) -> Self {
        Self {
            timestamp_usec: chat.timestamp_usec,
            author_external_channel_id: chat.author_external_channel_id,
            author_name: chat.author_name,
            message: chat.message,
            is_moderator: chat.is_moderator,
            membership_months: chat.membership_months,
            category: chat.category.to_string(),
        }
    }
}
