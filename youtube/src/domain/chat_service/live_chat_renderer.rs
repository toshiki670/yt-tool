use crate::domain::{
    live_chat::item::renderers::live_chat_renderer::LiveChatRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl Into<SimpleChatEntity> for LiveChatRenderer {
    fn into(self) -> SimpleChatEntity {
        let is_moderator = if let Some(author_badges) = &self.author_badges {
            author_badges.has_moderator()
        } else {
            false
        };

        let membership_months = if let Some(author_badges) = &self.author_badges {
            author_badges.fetch_membership_months()
        } else {
            None
        };
        let membership_months = membership_months.unwrap_or("".to_string());

        SimpleChatEntity {
            id: self.id,
            posted_at: self.timestamp_usec.into(),
            author_external_channel_id: self.author_external_channel_id,
            author_name: self.author_name.into(),
            content: self.message.into(),
            is_moderator,
            membership_months,
            category: CategoryValue::ChatTextMessage,
        }
    }
}
