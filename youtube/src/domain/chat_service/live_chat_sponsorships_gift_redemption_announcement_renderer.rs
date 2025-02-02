use crate::domain::{
    live_chat::item::renderers::live_chat_sponsorships_gift_redemption_announcement_renderer::LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl From<Box<LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer>) -> Self {
        let is_moderator = if let Some(author_badges) = &val.author_badges {
            author_badges.has_moderator()
        } else {
            false
        };

        let membership_months = if let Some(author_badges) = &val.author_badges {
            author_badges.fetch_membership_months()
        } else {
            None
        };
        let membership_months = membership_months.unwrap_or("".to_string());

        SimpleChatEntity {
            id: val.id,
            posted_at: val.timestamp_usec.into(),
            author_external_channel_id: val.author_external_channel_id,
            author_name: val
                .author_name
                .map(|v| v.simple_text)
                .unwrap_or("".to_string()),
            content: val.message.into(),
            is_moderator,
            membership_months,
            category: CategoryValue::ChatTextMessage,
        }
    }
}
