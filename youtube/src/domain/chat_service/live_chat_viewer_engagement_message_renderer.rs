use crate::domain::{
    live_chat::item::renderers::live_chat_viewer_engagement_message_renderer::LiveChatViewerEngagementMessageRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<Box<LiveChatViewerEngagementMessageRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatViewerEngagementMessageRenderer>) -> Self {
        let mut content = Content::new();
        content.add("message", Some(String::from(val.message)));

        SimpleChatEntity {
            id: val.id,
            posted_at: Some(val.timestamp_usec.into()),
            author_external_channel_id: None,
            category: CategoryValue::ViewerEngagementMessage,
            author_name: Some("System Message".to_string()),
            content,
        }
    }
}
