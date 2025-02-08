use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CloseLiveChatActionPanelAction {
    pub skip_on_dismiss_command: bool,
    pub target_panel_id: String,
}
