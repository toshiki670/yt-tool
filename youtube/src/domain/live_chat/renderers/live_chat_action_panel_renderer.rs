use serde::{Deserialize, Serialize};

use super::poll_renderer::PollRenderer;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatActionPanelRenderer {
    pub contents: Contents,
    pub id: String,
    pub target_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub poll_renderer: Box<PollRenderer>,
}
