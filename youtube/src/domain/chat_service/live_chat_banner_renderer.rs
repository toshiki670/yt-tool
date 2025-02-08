use crate::domain::{
    live_chat::item::renderers::live_chat_banner_renderer::LiveChatBannerRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<Box<LiveChatBannerRenderer>> for Vec<SimpleChatEntity> {
    fn from(val: Box<LiveChatBannerRenderer>) -> Self {
        let posted_at = Some(
            val.contents
                .live_chat_text_message_renderer
                .timestamp_usec
                .clone()
                .into(),
        );
        let author_name = Some(val.header.live_chat_banner_header_renderer.text.into());

        let mut content = Content::new();
        content.add(
            "TextMessageID",
            Some(val.contents.live_chat_text_message_renderer.id.clone()),
        );

        vec![
            SimpleChatEntity {
                id: val.action_id,
                author_external_channel_id: None,
                posted_at,
                category: CategoryValue::Banner,
                author_name,
                content,
            },
            SimpleChatEntity::from(Box::new(val.contents.live_chat_text_message_renderer)),
        ]
    }
}
