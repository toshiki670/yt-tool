use super::values::{
    accessibility::Accessibility, author_badge::AuthorBadges,
    context_menu_endpoint::ContextMenuEndpoint, text::Text, thumbnails::Thumbnails,
    timestamp_usec::TimestampUsec,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTextMessageRenderer {
    pub author_badges: Option<AuthorBadges>,
    pub author_external_channel_id: String,
    pub author_name: Option<Text>,
    pub author_photo: Thumbnails,
    pub context_menu_accessibility: Option<Accessibility>,
    pub context_menu_endpoint: Option<ContextMenuEndpoint>,
    pub id: String,
    pub message: Text,
    pub timestamp_usec: TimestampUsec,
}
