use crate::domain::{
    live_chat::item::renderers::live_chat_viewer_engagement_message_renderer::LiveChatViewerEngagementMessageRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl From<LiveChatViewerEngagementMessageRenderer> for SimpleChatEntity {
    fn from(val: LiveChatViewerEngagementMessageRenderer) -> Self {
        SimpleChatEntity {
            id: val.id,
            posted_at: val.timestamp_usec.into(),
            author_external_channel_id: "".to_string(),
            author_name: "".to_string(),
            content: val.message.into(),
            is_moderator: false,
            membership_months: "".to_string(),
            category: CategoryValue::ChatViewerEngagementMessage,
        }
    }
}
