use crate::domain::{
    live_chat::item::renderers::live_chat_text_message_renderer::LiveChatTextMessageRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<Box<LiveChatTextMessageRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatTextMessageRenderer>) -> Self {
        let author_name = val
            .author_name
            .map(|v| v.simple_text)
            .unwrap_or("".to_string());

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
            author_external_channel_id: val.author_external_channel_id,
            posted_at: val.timestamp_usec.into(),
            category: CategoryValue::TextMessage,
            author_name,
            content,
        }
    }
}
