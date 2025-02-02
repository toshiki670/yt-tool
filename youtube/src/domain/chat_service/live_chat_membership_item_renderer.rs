use crate::domain::{
    live_chat::item::renderers::live_chat_membership_item_renderer::LiveChatMembershipItemRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<Box<LiveChatMembershipItemRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatMembershipItemRenderer>) -> Self {
        let author_name = val
            .author_name
            .map(|v| v.simple_text)
            .unwrap_or("".to_string());

        let mut content = Content::new();
        content.add("message", Some(String::from(val.header_subtext)));

        if val.author_badges.has_moderator() {
            content.add("Moderator", None);
        }

        if let Some(months) = val.author_badges.fetch_membership_months() {
            content.add("membershipMonths", Some(months));
        }

        SimpleChatEntity {
            id: val.id,
            posted_at: val.timestamp_usec.into(),
            author_external_channel_id: val.author_external_channel_id,
            category: CategoryValue::MembershipItem,
            author_name,
            content,
        }
    }
}
