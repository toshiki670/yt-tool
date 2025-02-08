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
    pub tracking_params: Option<String>,
    pub live_chat_poll_id: String,
    pub header: Header,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub text: Text,
    pub selected: bool,
    pub vote_ratio: Option<f64>,
    pub vote_percentage: Option<Text>,
    pub select_service_endpoint: Option<SelectServiceEndpoint>,
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
    pub send_post: bool,
    pub api_url: String,
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
    pub poll_question: Text,
    pub thumbnail: Thumbnails,
    pub metadata_text: Text,
    pub live_chat_poll_type: String,
    pub context_menu_button: ContextMenuButton,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextMenuButton {
    pub button_renderer: ButtonRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer {
    pub icon: Icon,
    pub accessibility: Label,
    pub tracking_params: Option<String>,
    pub accessibility_data: Accessibility,
    pub target_id: String,
    pub command: ContextMenuEndpoint,
}
