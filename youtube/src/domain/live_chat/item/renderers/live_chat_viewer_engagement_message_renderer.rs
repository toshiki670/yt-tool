use super::values::{icon::Icon, text::Text, timestamp_usec::TimestampUsec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatViewerEngagementMessageRenderer {
    pub id: String,
    pub timestamp_usec: Option<TimestampUsec>,
    pub icon: Icon,
    pub message: Text,
    pub action_button: Option<serde_json::Value>,
    pub tracking_params: String,
    pub context_menu_endpoint: Option<serde_json::Value>,
    pub context_menu_accessibility: Option<serde_json::Value>,
}
