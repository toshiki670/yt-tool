use super::values::{icon::Icon, text::Text, timestamp_usec::TimestampUsec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatViewerEngagementMessageRenderer {
    pub action_button: Option<serde_json::Value>,
    pub context_menu_accessibility: Option<serde_json::Value>,
    pub context_menu_endpoint: Option<serde_json::Value>,
    pub icon: Icon,
    pub id: String,
    pub message: Text,
    pub timestamp_usec: Option<TimestampUsec>,
    pub tracking_params: Option<String>,
}
