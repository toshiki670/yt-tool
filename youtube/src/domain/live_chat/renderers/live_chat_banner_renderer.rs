use serde::{Deserialize, Serialize};

use super::live_chat_text_message_renderer::LiveChatTextMessageRenderer;
use crate::domain::live_chat::values::{
    accessibility::{Accessibility, Label},
    icon::Icon,
    ignore_navigation::IgnoreNavigation,
    text::Text,
    web_command_metadata::WebCommandMetadata,
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
    pub command: Command,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Command {
    pub click_tracking_params: String,
    pub command_metadata: WebCommandMetadata<IgnoreNavigation>,
    pub live_chat_item_context_menu_endpoint: LiveChatItemContextMenuEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatItemContextMenuEndpoint {
    pub params: String,
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
