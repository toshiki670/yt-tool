use serde::{Deserialize, Serialize};

use crate::domain::live_chat::values::{
    accessibility::Accessibility, context_menu_endpoint::ContextMenuEndpoint,
    ignore_navigation::IgnoreNavigation, web_command_metadata::WebCommandMetadata,
};

use super::values::{text::Text, thumbnails::Thumbnails, timestamp_usec::TimestampUsec};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatTickerPaidStickerItemRenderer {
    pub id: String,
    pub author_photo: Thumbnails,
    pub start_background_color: i64,
    pub end_background_color: i64,
    pub duration_sec: i64,
    pub full_duration_sec: i64,
    pub show_item_endpoint: ShowItemEndpoint,
    pub author_external_channel_id: String,
    pub ticker_thumbnails: Vec<Thumbnails>,
    pub tracking_params: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShowItemEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: WebCommandMetadata<IgnoreNavigation>,
    pub show_live_chat_item_endpoint: ShowLiveChatItemEndpoint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShowLiveChatItemEndpoint {
    pub renderer: Renderer,
    pub tracking_params: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Renderer {
    pub live_chat_paid_sticker_renderer: LiveChatPaidStickerRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatPaidStickerRenderer {
    pub id: String,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub context_menu_accessibility: Accessibility,
    pub timestamp_usec: TimestampUsec,
    pub author_photo: Thumbnails,
    pub author_name: Text,
    pub author_external_channel_id: String,
    pub timestamp_text: Option<Text>,
    pub sticker: Thumbnails,
    pub money_chip_background_color: i64,
    pub money_chip_text_color: i64,
    pub purchase_amount_text: Text,
    pub sticker_display_width: i64,
    pub sticker_display_height: i64,
    pub background_color: i64,
    pub author_name_text_color: i64,
    pub author_badges: Option<serde_json::Value>,
    pub tracking_params: String,
    #[serde(rename = "isV2Style")]
    pub is_v2style: bool,
    pub header_overlay_image: Option<Thumbnails>,
    pub pdg_purchased_novelty_logging_directives: Option<serde_json::Value>,
    pub lower_bumper: Option<serde_json::Value>,
}
