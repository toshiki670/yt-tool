use crate::domain::live_chat::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AddLiveChatTickerItemAction {
    pub duration_sec: String,
    pub item: Item,
}
