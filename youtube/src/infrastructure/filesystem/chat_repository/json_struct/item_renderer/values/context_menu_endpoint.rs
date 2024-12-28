use serde::{Deserialize, Serialize};

use super::web_command_metadata::WebCommandMetadata;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextMenuEndpoint {
    pub click_tracking_params: Option<String>,
    pub command_metadata: WebCommandMetadata,
    pub live_chat_item_context_menu_endpoint: Params,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub params: String,
}
