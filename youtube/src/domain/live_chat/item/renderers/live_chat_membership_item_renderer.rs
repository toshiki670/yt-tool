use serde::{Deserialize, Serialize};

use super::values::{
    accessibility::Accessibility, author_badge::AuthorBadges,
    context_menu_endpoint::ContextMenuEndpoint, message::Message, simple_text::SimpleText,
    text::Text, thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatMembershipItemRenderer {
    pub id: String,
    pub timestamp_usec: TimestampUsec,
    pub timestamp_text: SimpleText,
    pub author_external_channel_id: String,
    pub header_subtext: Text,
    pub message: Option<Message>,
    pub author_name: SimpleText,
    pub author_photo: Thumbnails,
    pub author_badges: AuthorBadges,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub context_menu_accessibility: Accessibility,
    pub tracking_params: String,
}
