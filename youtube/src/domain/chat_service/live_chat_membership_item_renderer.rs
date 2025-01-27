use crate::domain::{
    live_chat::item::renderers::live_chat_membership_item_renderer::LiveChatMembershipItemRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl Into<SimpleChatEntity> for LiveChatMembershipItemRenderer {
    fn into(self) -> SimpleChatEntity {
        let is_moderator = self.author_badges.has_moderator();
        let membership_months = self.author_badges.fetch_membership_months();
        let membership_months = membership_months.unwrap_or("".to_string());

        SimpleChatEntity {
            id: self.id,
            posted_at: self.timestamp_usec.into(),
            author_external_channel_id: self.author_external_channel_id,
            author_name: self.author_name.into(),
            content: self.header_subtext.into(),
            is_moderator,
            membership_months,
            category: CategoryValue::ChatMembershipItem,
        }
    }
}
