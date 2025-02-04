use serde::{Deserialize, Serialize};

use super::values::{
    accessibility::Accessibility, author_badge::AuthorBadges,
    context_menu_endpoint::ContextMenuEndpoint, text::Text, thumbnails::Thumbnails,
    timestamp_usec::TimestampUsec,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatMembershipItemRenderer {
    pub id: String,
    pub timestamp_usec: TimestampUsec,
    pub timestamp_text: Option<Text>,
    pub author_external_channel_id: String,
    pub header_subtext: Text,
    pub message: Option<Text>,
    pub author_name: Option<Text>,
    pub author_photo: Thumbnails,
    pub author_badges: AuthorBadges,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub context_menu_accessibility: Accessibility,
    pub tracking_params: String,
    pub header_primary_text: Option<Text>,
}
