use super::{
    values::{
        accessibility::Accessibility, author_badge::AuthorBadge,
        context_menu_endpoint::ContextMenuEndpoint, icon::IconType, message::Message,
        simple_text::SimpleText, thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
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
        let is_moderator = if let Some(author_badges) = self.author_badges {
            author_badges.iter().any(|badge| {
                if let Some(icon) = &badge.live_chat_author_badge_renderer.icon {
                    icon.is_moderator()
                } else {
                    false
                }
            })
        } else {
            false
        };

        CommonRenderer {
            id: self.id,
            timestamp_usec: self.timestamp_usec.into(),
            author_external_channel_id: self.author_external_channel_id,
            author_name: self.author_name.into(),
            message: self.message.into(),
            is_moderator,
            membership_months: "0".to_string(),
        }
    }
}
