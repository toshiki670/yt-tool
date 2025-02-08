use serde::{Deserialize, Serialize};

use super::values::{
    accessibility::Accessibility, author_badge::AuthorBadges,
    context_menu_endpoint::ContextMenuEndpoint, text::Text, thumbnails::Thumbnails,
    timestamp_usec::TimestampUsec,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatMembershipItemRenderer {
    pub author_badges: AuthorBadges,
    pub author_external_channel_id: String,
    pub author_name: Option<Text>,
    pub author_photo: Thumbnails,
    pub context_menu_accessibility: Accessibility,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub empty: Option<bool>,
    pub header_primary_text: Option<Text>,
    pub header_subtext: Text,
    pub id: String,
    pub message: Option<Text>,
    pub timestamp_text: Option<Text>,
    pub timestamp_usec: TimestampUsec,
    pub tracking_params: String,
}
