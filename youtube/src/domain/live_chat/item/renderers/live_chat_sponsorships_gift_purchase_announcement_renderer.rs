use serde::{Deserialize, Serialize};

use super::values::{
    message::Message, simple_text::SimpleText, thumbnails::Thumbnails,
    timestamp_usec::TimestampUsec,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer {
    pub id: String,
    pub timestamp_usec: TimestampUsec,
    pub author_external_channel_id: String,
    pub header: Header,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub live_chat_sponsorships_header_renderer: LiveChatSponsorshipsHeaderRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatSponsorshipsHeaderRenderer {
    pub author_name: SimpleText,
    pub author_photo: Thumbnails,
    pub primary_text: Message,
    pub context_menu_endpoint: serde_json::Value,
    pub context_menu_accessibility: serde_json::Value,
    pub image: serde_json::Value,
}
