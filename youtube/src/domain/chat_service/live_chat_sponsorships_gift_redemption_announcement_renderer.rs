use crate::domain::{
    live_chat::renderers::live_chat_sponsorships_gift_redemption_announcement_renderer::LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer> for SimpleChatEntity {
    fn from(val: LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer) -> Self {
        let author_name = val.author_name.map(|v| v.into());

        let mut content = Content::new();
        content.add("message", Some(String::from(val.message)));

        if let Some(author_badges) = &val.author_badges {
            if author_badges.has_moderator() {
                content.add("Moderator", None);
            }
        }

        if let Some(author_badges) = &val.author_badges {
            if let Some(months) = author_badges.fetch_membership_months() {
                content.add("membershipMonths", Some(months));
            }
        }

        SimpleChatEntity {
            id: val.id,
            author_external_channel_id: Some(val.author_external_channel_id),
            posted_at: Some(val.timestamp_usec.into()),
            category: CategoryValue::SponsorshipsGiftRedemptionAnnouncement,
            author_name,
            content,
        }
    }
}
