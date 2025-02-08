// https://transform.tools/json-to-rust-serde
pub(crate) mod actions;
pub(crate) mod commands;
pub(crate) mod item;
pub(crate) mod renderers;
pub(crate) mod values;

use actions::Actions;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatEntity {
    pub click_tracking_params: Option<String>,
    pub is_live: Option<bool>,
    pub replay_chat_item_action: Actions,
    pub video_offset_time_msec: Option<String>,
}
