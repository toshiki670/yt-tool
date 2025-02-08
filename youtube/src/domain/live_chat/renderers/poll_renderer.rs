use crate::domain::live_chat::values::{
    accessibility::{Accessibility, Label},
    context_menu_endpoint::ContextMenuEndpoint,
    icon::Icon,
    text::Text,
    thumbnails::Thumbnails,
    web_command_metadata::WebCommandMetadata,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollRenderer {
    pub choices: Vec<Choice>,
    pub header: Header,
    pub live_chat_poll_id: String,
    pub tracking_params: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub select_service_endpoint: Option<SelectServiceEndpoint>,
    pub selected: bool,
    pub text: Text,
    pub vote_percentage: Option<Text>,
    pub vote_ratio: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectServiceEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: WebCommandMetadata<SelectServiceMetadata>,
    pub send_live_chat_vote_endpoint: SendLiveChatVoteEndpoint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectServiceMetadata {
    pub api_url: String,
    pub send_post: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLiveChatVoteEndpoint {
    pub params: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub poll_header_renderer: PollHeaderRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollHeaderRenderer {
    pub context_menu_button: ContextMenuButton,
    pub live_chat_poll_type: String,
    pub metadata_text: Text,
    pub poll_question: Text,
    pub thumbnail: Thumbnails,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextMenuButton {
    pub button_renderer: ButtonRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer {
    pub accessibility_data: Accessibility,
    pub accessibility: Label,
    pub command: ContextMenuEndpoint,
    pub icon: Icon,
    pub target_id: String,
    pub tracking_params: Option<String>,
}
