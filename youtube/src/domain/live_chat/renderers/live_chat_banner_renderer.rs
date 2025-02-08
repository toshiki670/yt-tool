use serde::{Deserialize, Serialize};

use super::live_chat_text_message_renderer::LiveChatTextMessageRenderer;
use crate::domain::live_chat::values::{
    accessibility::{Accessibility, Label},
    context_menu_endpoint::ContextMenuEndpoint,
    icon::Icon,
    text::Text,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatBannerRenderer {
    pub header: Header,
    pub contents: Contents,
    pub action_id: String,
    pub viewer_is_creator: bool,
    pub target_id: String,
    pub is_stackable: bool,
    pub background_type: String,
    pub banner_properties: BannerProperties,
    pub banner_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Header {
    pub live_chat_banner_header_renderer: LiveChatBannerHeaderRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatBannerHeaderRenderer {
    pub icon: Icon,
    pub text: Text,
    pub context_menu_button: ContextMenuButton,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextMenuButton {
    pub button_renderer: ButtonRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ButtonRenderer {
    pub icon: Icon,
    pub accessibility: Label,
    pub tracking_params: String,
    pub accessibility_data: Accessibility,
    pub command: ContextMenuEndpoint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Contents {
    pub live_chat_text_message_renderer: LiveChatTextMessageRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BannerProperties {
    pub auto_collapse_delay: AutoCollapseDelay,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AutoCollapseDelay {
    pub seconds: String,
}
