use serde::{Deserialize, Serialize};

use super::{
    live_chat_paid_sticker_renderer::LiveChatPaidStickerRenderer, values::thumbnails::Thumbnails,
};
use crate::domain::live_chat::values::{
    ignore_navigation::IgnoreNavigation, web_command_metadata::WebCommandMetadata,
};

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
