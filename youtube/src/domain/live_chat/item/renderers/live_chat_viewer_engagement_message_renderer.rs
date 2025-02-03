use super::values::{icon::Icon, text::Text, timestamp_usec::TimestampUsec};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatViewerEngagementMessageRenderer {
    pub id: String,
    pub timestamp_usec: TimestampUsec,
    pub icon: Icon,
    pub message: Text,
    pub action_button: Option<serde_json::Value>,
    pub tracking_params: String,
}
