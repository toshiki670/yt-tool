mod action;

use action::ReplayChatItemAction;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub is_live: bool,
    pub replay_chat_item_action: ReplayChatItemAction,
    pub video_offset_time_msec: String,
}
