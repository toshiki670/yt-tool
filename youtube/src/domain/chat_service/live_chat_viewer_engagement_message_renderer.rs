use crate::domain::{
    live_chat::renderers::live_chat_viewer_engagement_message_renderer::LiveChatViewerEngagementMessageRenderer,
    simple_chat::{CategoryValue, Content, PostedAtValue, SimpleChatEntity},
};

impl From<Box<LiveChatViewerEngagementMessageRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatViewerEngagementMessageRenderer>) -> Self {
        let posted_at = val.timestamp_usec.map(PostedAtValue::from);
        let mut content = Content::new();
        content.add("message", Some(String::from(val.message)));

        SimpleChatEntity {
            id: val.id,
            posted_at,
            author_external_channel_id: None,
            category: CategoryValue::ViewerEngagementMessage,
            author_name: Some("System Message".to_string()),
            content,
        }
    }
}
