use super::values::{
    accessibility::Accessibility, author_badge::AuthorBadges,
    context_menu_endpoint::ContextMenuEndpoint, message::Message, text::Text,
    thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer {
    pub author_badges: Option<AuthorBadges>,
    pub author_external_channel_id: String,
    pub author_name: Option<Text>,
    pub author_photo: Thumbnails,
    pub context_menu_accessibility: Accessibility,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub id: String,
    pub message: Message,
    pub timestamp_usec: TimestampUsec,
}
