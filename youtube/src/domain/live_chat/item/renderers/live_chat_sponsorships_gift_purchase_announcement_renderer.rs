use serde::{Deserialize, Serialize};

use super::values::{text::Text, thumbnails::Thumbnails, timestamp_usec::TimestampUsec};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer {
    pub id: String,
    pub timestamp_usec: TimestampUsec,
    pub author_external_channel_id: String,
    pub header: Header,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Header {
    pub live_chat_sponsorships_header_renderer: LiveChatSponsorshipsHeaderRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatSponsorshipsHeaderRenderer {
    pub author_name: Text,
    pub author_photo: Thumbnails,
    pub primary_text: Text,
    pub context_menu_endpoint: serde_json::Value,
    pub context_menu_accessibility: serde_json::Value,
    pub image: serde_json::Value,
}
