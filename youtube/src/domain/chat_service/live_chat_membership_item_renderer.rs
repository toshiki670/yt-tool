use crate::domain::{
    live_chat::item::renderers::live_chat_membership_item_renderer::LiveChatMembershipItemRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl From<Box<LiveChatMembershipItemRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatMembershipItemRenderer>) -> Self {
        let is_moderator = val.author_badges.has_moderator();
        let membership_months = val.author_badges.fetch_membership_months();
        let membership_months = membership_months.unwrap_or("".to_string());

        SimpleChatEntity {
            id: val.id,
            posted_at: val.timestamp_usec.into(),
            author_external_channel_id: val.author_external_channel_id,
            author_name: val.author_name.into(),
            content: val.header_subtext.into(),
            is_moderator,
            membership_months,
            category: CategoryValue::ChatMembershipItem,
        }
    }
}
