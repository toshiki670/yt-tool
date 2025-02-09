use serde::{Deserialize, Serialize};

use crate::domain::live_chat::item::Item;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReplaceChatItemAction {
    pub target_item_id: String,
    pub replacement_item: Item,
}
