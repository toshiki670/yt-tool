use serde::{Deserialize, Serialize};

use super::values::{icon::Icon, message::Message, timestamp_usec::TimestampUsec};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatViewerEngagementMessageRenderer {
    pub id: String,
    pub timestamp_usec: TimestampUsec,
    pub icon: Icon,
    pub message: Message,
    pub action_button: Option<serde_json::Value>,
    pub tracking_params: String,
}
