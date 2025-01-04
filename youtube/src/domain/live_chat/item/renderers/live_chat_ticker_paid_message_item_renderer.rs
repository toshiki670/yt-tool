use super::{
    live_chat_paid_message_renderer::LiveChatPaidMessageRenderer,
    values::{
        engagement_panel_command::EngagementPanelCommand, ignore_navigation::IgnoreNavigation,
        simple_text::SimpleText, thumbnails::Thumbnails, web_command_metadata::WebCommandMetadata,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTickerPaidMessageItemRenderer {
    pub id: String,
    pub amount_text_color: i64,
    pub start_background_color: i64,
    pub end_background_color: i64,
    pub author_photo: Thumbnails,
    pub duration_sec: i64,
    pub show_item_endpoint: ShowItemEndpoint,
    pub author_external_channel_id: String,
    pub full_duration_sec: i64,
    pub tracking_params: String,
    pub author_username: SimpleText,
    pub animation_origin: String,
    pub open_engagement_panel_command: EngagementPanelCommand,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowItemEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: WebCommandMetadata<IgnoreNavigation>,
    pub show_live_chat_item_endpoint: ShowLiveChatItemEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowLiveChatItemEndpoint {
    pub renderer: Renderer,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Renderer {
    pub live_chat_paid_message_renderer: LiveChatPaidMessageRenderer,
}
