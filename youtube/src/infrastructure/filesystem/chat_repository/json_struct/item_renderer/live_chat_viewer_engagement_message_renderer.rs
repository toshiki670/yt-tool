use super::{
    values::{
        icon::Icon, message::Message, simple_text::SimpleText, timestamp_usec::TimestampUsec,
    },
    CommonRenderer,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatViewerEngagementMessageRenderer {
    pub id: String,
    pub timestamp_usec: TimestampUsec,
    pub icon: Icon,
    pub message: Message,
    pub action_button: serde_json::Value,
    pub tracking_params: String,
}

impl Into<CommonRenderer> for LiveChatViewerEngagementMessageRenderer {
    fn into(self) -> CommonRenderer {
        CommonRenderer {
            id: self.id,
            timestamp_usec: self.timestamp_usec.into(),
            author_external_channel_id: "".to_string(),
            author_name: SimpleText::default().into(),
            message: self.message.into(),
            is_moderator: false,
            membership_months: "0".to_string(),
        }
    }
}
