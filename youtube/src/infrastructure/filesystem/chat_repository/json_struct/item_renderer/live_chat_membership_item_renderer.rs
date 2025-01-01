use serde::{Deserialize, Serialize};

use super::{
    values::{
        accessibility::Accessibility, author_badge::AuthorBadges,
        context_menu_endpoint::ContextMenuEndpoint, message::Message, simple_text::SimpleText,
        thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
    },
    CommonRenderer,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatMembershipItemRenderer {
    pub id: String,
    pub timestamp_usec: TimestampUsec,
    pub timestamp_text: SimpleText,
    pub author_external_channel_id: String,
    pub header_subtext: Message,
    pub author_name: SimpleText,
    pub author_photo: Thumbnails,
    pub author_badges: AuthorBadges,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub context_menu_accessibility: Accessibility,
    pub tracking_params: String,
}

impl Into<CommonRenderer> for LiveChatMembershipItemRenderer {
    fn into(self) -> CommonRenderer {
        let is_moderator = self.author_badges.has_moderator();
        let membership_months = self.author_badges.fetch_membership_months();
        let membership_months = membership_months.unwrap_or("".to_string());

        CommonRenderer {
            id: self.id,
            timestamp_usec: self.timestamp_usec.into(),
            author_external_channel_id: self.author_external_channel_id,
            author_name: self.author_name.into(),
            message: self.header_subtext.into(),
            is_moderator,
            membership_months: membership_months,
        }
    }
}
