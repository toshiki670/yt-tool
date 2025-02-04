use serde::{Deserialize, Serialize};

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
    pub command_metadata: CommandMetadata,
    pub show_live_chat_item_endpoint: ShowLiveChatItemEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CommandMetadata {
    pub web_command_metadata: WebCommandMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WebCommandMetadata {
    pub ignore_navigation: bool,
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
    pub context_menu_accessibility: ContextMenuAccessibility,
    pub timestamp_usec: TimestampUsec,
    pub author_photo: AuthorPhoto2,
    pub author_name: Text,
    pub author_external_channel_id: String,
    pub timestamp_text: Option<TimestampText>,
    pub sticker: Sticker,
    pub money_chip_background_color: i64,
    pub money_chip_text_color: i64,
    pub purchase_amount_text: PurchaseAmountText,
    pub sticker_display_width: i64,
    pub sticker_display_height: i64,
    pub background_color: i64,
    pub author_name_text_color: i64,
    pub tracking_params: String,
    #[serde(rename = "isV2Style")]
    pub is_v2style: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextMenuEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata2,
    pub live_chat_item_context_menu_endpoint: LiveChatItemContextMenuEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CommandMetadata2 {
    pub web_command_metadata: WebCommandMetadata2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WebCommandMetadata2 {
    pub ignore_navigation: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatItemContextMenuEndpoint {
    pub params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextMenuAccessibility {
    pub accessibility_data: AccessibilityData2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AccessibilityData2 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AuthorPhoto2 {
    pub thumbnails: Vec<Thumbnail2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thumbnail2 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AuthorName {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TimestampText {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Sticker {
    pub thumbnails: Vec<Thumbnail3>,
    pub accessibility: Accessibility2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thumbnail3 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Accessibility2 {
    pub accessibility_data: AccessibilityData3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AccessibilityData3 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PurchaseAmountText {
    pub simple_text: String,
}
