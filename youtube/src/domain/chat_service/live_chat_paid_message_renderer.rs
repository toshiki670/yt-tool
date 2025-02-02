use crate::domain::{
    live_chat::item::renderers::live_chat_paid_message_renderer::LiveChatPaidMessageRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl LiveChatPaidMessageRenderer {
    pub fn message_text(&self) -> String {
        if let Some(message) = &self.message {
            format!("{}: {}", self.purchase_amount_text.simple_text, message)
        } else {
            self.purchase_amount_text.simple_text.clone()
        }
    }
}

impl From<Box<LiveChatPaidMessageRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatPaidMessageRenderer>) -> Self {
        let author_name = val
            .author_name
            .clone()
            .map(|v| v.simple_text)
            .unwrap_or("".to_string());

        let mut content = Content::new();
        content.add("message", Some(String::from(val.message_text())));

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
            posted_at: val.timestamp_usec.into(),
            author_external_channel_id: val.author_external_channel_id,
            category: CategoryValue::PaidMessage,
            author_name,
            content,
        }
    }
}
