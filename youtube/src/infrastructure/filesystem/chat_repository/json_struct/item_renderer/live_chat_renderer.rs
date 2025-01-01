use super::{
    values::{
        accessibility::Accessibility, author_badge::AuthorBadges,
        context_menu_endpoint::ContextMenuEndpoint, message::Message, simple_text::SimpleText,
        thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
    },
    CommonRenderer,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatRenderer {
    pub author_badges: Option<AuthorBadges>,
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
        let is_moderator = if let Some(author_badges) = &self.author_badges {
            author_badges.has_moderator()
        } else {
            false
        };

        let membership_months = if let Some(author_badges) = &self.author_badges {
            author_badges.fetch_membership_months()
        } else {
            None
        };
        let membership_months = membership_months.unwrap_or("".to_string());

        CommonRenderer {
            id: self.id,
            timestamp_usec: self.timestamp_usec.into(),
            author_external_channel_id: self.author_external_channel_id,
            author_name: self.author_name.into(),
            message: self.message.into(),
            is_moderator,
            membership_months: membership_months,
        }
    }
}
