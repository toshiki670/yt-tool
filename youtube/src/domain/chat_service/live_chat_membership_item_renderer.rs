use crate::domain::{
    live_chat::renderers::live_chat_membership_item_renderer::LiveChatMembershipItemRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<LiveChatMembershipItemRenderer> for SimpleChatEntity {
    fn from(val: LiveChatMembershipItemRenderer) -> Self {
        let author_name = val.author_name.map(|v| v.into());

        let mut content = Content::new();
        if let Some(message) = val.message {
            content.add("message", Some(String::from(message)));
        }

        if let Some(header_subtext) = val.header_subtext {
            content.add("headerSubtext", Some(header_subtext.into()));
        }

        if val.author_badges.has_moderator() {
            content.add("Moderator", None);
        }

        if let Some(months) = val.author_badges.fetch_membership_months() {
            content.add("membershipMonths", Some(months));
        }

        SimpleChatEntity {
            id: val.id,
            posted_at: Some(val.timestamp_usec.into()),
            author_external_channel_id: Some(val.author_external_channel_id),
            category: CategoryValue::MembershipItem,
            author_name,
            content,
        }
    }
}
