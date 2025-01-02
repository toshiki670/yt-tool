use crate::domain::{
    live_chat::item::renderers::live_chat_sponsorships_gift_purchase_announcement_renderer::LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl Into<SimpleChatEntity> for LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer {
    fn into(self) -> SimpleChatEntity {
        SimpleChatEntity {
            id: self.id,
            posted_at: self.timestamp_usec.into(),
            author_external_channel_id: self.author_external_channel_id,
            author_name: self
                .header
                .live_chat_sponsorships_header_renderer
                .author_name
                .into(),
            content: self
                .header
                .live_chat_sponsorships_header_renderer
                .primary_text
                .into(),
            is_moderator: false,
            membership_months: "".to_string(),
            category: CategoryValue::ChatSponsorshipsGiftPurchaseAnnouncement,
        }
    }
}
