use serde::{Deserialize, Serialize};

use crate::domain::live_chat::values::{
    accessibility::Accessibility, context_menu_endpoint::ContextMenuEndpoint, text::Text,
    thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatPaidStickerRenderer {
    pub author_badges: Option<serde_json::Value>,
    pub author_external_channel_id: String,
    pub author_name_text_color: i64,
    pub author_name: Text,
    pub author_photo: Thumbnails,
    pub background_color: i64,
    pub context_menu_accessibility: Accessibility,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub header_overlay_image: Option<Thumbnails>,
    pub id: String,
    #[serde(rename = "isV2Style")]
    pub is_v2style: bool,
    pub lower_bumper: Option<serde_json::Value>,
    pub money_chip_background_color: i64,
    pub money_chip_text_color: i64,
    pub pdg_purchased_novelty_logging_directives: Option<serde_json::Value>,
    pub purchase_amount_text: Text,
    pub sticker_display_height: i64,
    pub sticker_display_width: i64,
    pub sticker: Thumbnails,
    pub timestamp_text: Option<Text>,
    pub timestamp_usec: TimestampUsec,
    pub tracking_params: String,
}
