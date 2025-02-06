use crate::domain::{
    live_chat::item::renderers::live_chat_paid_message_renderer::LiveChatPaidMessageRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<Box<LiveChatPaidMessageRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatPaidMessageRenderer>) -> Self {
        let author_name = val.author_name.clone().map(|v| v.into());

        let mut content = Content::new();
        if let Some(message) = val.message {
            content.add("message", Some(message.into()));
        }

        content.add(
            "purchaseAmount",
            Some(val.purchase_amount_text.clone().into()),
        );

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
            posted_at: Some(val.timestamp_usec.into()),
            author_external_channel_id: Some(val.author_external_channel_id),
            category: CategoryValue::PaidMessage,
            author_name,
            content,
        }
    }
}
