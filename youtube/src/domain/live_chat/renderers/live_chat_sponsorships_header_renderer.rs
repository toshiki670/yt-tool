use super::values::{text::Text, thumbnails::Thumbnails};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatSponsorshipsHeaderRenderer {
    pub author_badges: Option<serde_json::Value>,
    pub author_name: Text,
    pub author_photo: Thumbnails,
    pub context_menu_accessibility: serde_json::Value,
    pub context_menu_endpoint: serde_json::Value,
    pub image: serde_json::Value,
    pub primary_text: Text,
}
