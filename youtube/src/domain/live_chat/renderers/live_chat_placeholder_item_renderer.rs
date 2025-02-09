use serde::{Deserialize, Serialize};

use crate::domain::live_chat::values::timestamp_usec::TimestampUsec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatPlaceholderItemRenderer {
    pub id: String,
    pub timestamp_usec: TimestampUsec,
}
