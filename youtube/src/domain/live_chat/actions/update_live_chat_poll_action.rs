use crate::domain::live_chat::renderers::poll_renderer::PollRenderer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UpdateLiveChatPollAction {
    pub poll_to_update: PollToUpdate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PollToUpdate {
    pub poll_renderer: Box<PollRenderer>,
}
