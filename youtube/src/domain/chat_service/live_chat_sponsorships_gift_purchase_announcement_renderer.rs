use crate::domain::{
    live_chat::renderers::live_chat_sponsorships_gift_purchase_announcement_renderer::LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<Box<LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer>) -> Self {
        let author_name = Some(
            val.header
                .live_chat_sponsorships_header_renderer
                .author_name
                .into(),
        );

        let mut content = Content::new();

        let message = val
            .header
            .live_chat_sponsorships_header_renderer
            .primary_text;

        content.add("message", Some(String::from(message)));

        SimpleChatEntity {
            id: val.id,
            posted_at: Some(val.timestamp_usec.into()),
            author_external_channel_id: Some(val.author_external_channel_id),
            category: CategoryValue::SponsorshipsGiftPurchaseAnnouncement,
            author_name,
            content,
        }
    }
}
