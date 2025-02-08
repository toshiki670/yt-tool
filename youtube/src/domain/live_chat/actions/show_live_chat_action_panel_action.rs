use serde::{Deserialize, Serialize};

use crate::domain::live_chat::renderers::live_chat_action_panel_renderer::LiveChatActionPanelRenderer;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShowLiveChatActionPanelAction {
    pub panel_to_show: PanelToShow,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PanelToShow {
    pub live_chat_action_panel_renderer: Box<LiveChatActionPanelRenderer>,
}
