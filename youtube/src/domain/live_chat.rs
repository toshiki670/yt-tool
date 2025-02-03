// https://transform.tools/json-to-rust-serde
pub(crate) mod item;

use item::Item;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatEntity {
    pub click_tracking_params: Option<String>,
    pub is_live: Option<bool>,
    pub replay_chat_item_action: ReplayChatItemAction,
    pub video_offset_time_msec: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayChatItemAction {
    pub actions: Vec<Action>,
    pub video_offset_time_msec: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Action {
    pub add_chat_item_action: Option<AddChatItemAction>,
    pub add_live_chat_ticker_item_action: Option<AddLiveChatTickerItemAction>,
    pub click_tracking_params: Option<String>,
    pub live_chat_report_moderation_state_command: Option<serde_json::Value>,
    pub remove_chat_item_action: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddChatItemAction {
    pub client_id: Option<String>,
    pub item: Item,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLiveChatTickerItemAction {
    pub duration_sec: String,
    pub item: Item,
}
