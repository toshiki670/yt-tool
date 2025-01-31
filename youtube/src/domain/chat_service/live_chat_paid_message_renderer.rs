use crate::domain::{
    live_chat::item::renderers::live_chat_paid_message_renderer::LiveChatPaidMessageRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
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
        let message = val.message_text();

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
            author_name: val.author_name.into(),
            content: message,
            is_moderator,
            membership_months,
            category: CategoryValue::ChatPaidMessage,
        }
    }
}
