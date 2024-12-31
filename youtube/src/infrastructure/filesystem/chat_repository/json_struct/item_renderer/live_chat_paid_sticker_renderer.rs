use serde::{Deserialize, Serialize};

use super::values::{author_badge::AuthorBadge, simple_text::SimpleText};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatPaidStickerRenderer {
    pub id: String,
    pub context_menu_endpoint: serde_json::Value,
    pub context_menu_accessibility: serde_json::Value,
    pub timestamp_usec: String,
    pub author_photo: serde_json::Value,
    pub author_name: SimpleText,
    pub author_external_channel_id: String,
    pub sticker: serde_json::Value,
    pub author_badges: Option<Vec<AuthorBadge>>,
    pub money_chip_background_color: i64,
    pub money_chip_text_color: i64,
    pub purchase_amount_text: SimpleText,
    pub sticker_display_width: i64,
    pub sticker_display_height: i64,
    pub background_color: i64,
    pub author_name_text_color: i64,
    pub tracking_params: String,
    #[serde(rename = "isV2Style")]
    pub is_v2style: bool,
}
