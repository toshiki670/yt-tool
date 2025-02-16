use serde::{Deserialize, Serialize};

use crate::domain::live_chat::values::{text::Text, thumbnails::Thumbnails};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatBannerRedirectRenderer {
    pub banner_message: Text,
    pub author_photo: Thumbnails,
    pub inline_action_button: InlineActionButton,
    pub context_menu_button: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InlineActionButton {
    pub button_renderer: ButtonRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ButtonRenderer {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub text: Text,
    pub tracking_params: String,
    pub command: Command,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Command {
    pub click_tracking_params: String,
    pub command_metadata: serde_json::Value,
    pub watch_endpoint: WatchEndpoint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WatchEndpoint {
    pub video_id: String,
}
