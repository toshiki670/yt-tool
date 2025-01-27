use crate::domain::{
    live_chat::item::renderers::live_chat_sponsorships_gift_purchase_announcement_renderer::LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl From<LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer> for SimpleChatEntity {
    fn from(val: LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer) -> Self {
        SimpleChatEntity {
            id: val.id,
            posted_at: val.timestamp_usec.into(),
            author_external_channel_id: val.author_external_channel_id,
            author_name: val
                .header
                .live_chat_sponsorships_header_renderer
                .author_name
                .into(),
            content: val
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
