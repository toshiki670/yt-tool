use crate::domain::{live_chat::item::renderers::live_chat_viewer_engagement_message_renderer::LiveChatViewerEngagementMessageRenderer, simple_chat::{CategoryValue, SimpleChatEntity}};

impl Into<SimpleChatEntity> for LiveChatViewerEngagementMessageRenderer {
    fn into(self) -> SimpleChatEntity {
        SimpleChatEntity {
            id: self.id,
            posted_at: self.timestamp_usec.into(),
            author_external_channel_id: "".to_string(),
            author_name: "".to_string(),
            content: self.message.into(),
            is_moderator: false,
            membership_months: "".to_string(),
            category: CategoryValue::ChatViewerEngagementMessage,
        }
    }
}
