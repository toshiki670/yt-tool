use super::{
    values::{
        accessibility::Accessibility, author_badge::AuthorBadge,
        context_menu_endpoint::ContextMenuEndpoint, message::Message, simple_text::SimpleText,
        thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
    },
    CommonRenderer,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatRenderer {
    pub author_badges: Option<Vec<AuthorBadge>>,
    pub author_external_channel_id: String,
    pub author_name: SimpleText,
    pub author_photo: Thumbnails,
    pub context_menu_accessibility: Accessibility,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub id: String,
    pub message: Message,
    pub timestamp_usec: TimestampUsec,
}

impl Into<CommonRenderer> for LiveChatRenderer {
    fn into(self) -> CommonRenderer {
        CommonRenderer {
            timestamp_usec: self.timestamp_usec.into(),
            author_external_channel_id: self.author_external_channel_id,
            author_name: self.author_name.into(),
            message: self.message.into(),
        }
    }
}
