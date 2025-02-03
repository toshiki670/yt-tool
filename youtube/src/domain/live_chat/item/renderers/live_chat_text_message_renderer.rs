use super::values::{
    accessibility::Accessibility, author_badge::AuthorBadges,
    context_menu_endpoint::ContextMenuEndpoint, message::Message, simple_text::SimpleText,
    thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTextMessageRenderer {
    pub author_badges: Option<AuthorBadges>,
    pub author_external_channel_id: String,
    pub author_name: Option<SimpleText>,
    pub author_photo: Thumbnails,
    pub context_menu_accessibility: Option<Accessibility>,
    pub context_menu_endpoint: Option<ContextMenuEndpoint>,
    pub id: String,
    pub message: Message,
    pub timestamp_usec: TimestampUsec,
}
