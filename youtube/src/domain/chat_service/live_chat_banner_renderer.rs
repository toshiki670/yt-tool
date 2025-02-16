use crate::domain::{
    live_chat::renderers::live_chat_banner_renderer::{Contents, LiveChatBannerRenderer},
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<LiveChatBannerRenderer> for Vec<SimpleChatEntity> {
    fn from(val: LiveChatBannerRenderer) -> Self {
        let author_name = if let Some(h) = val.header {
            Some(h.live_chat_banner_header_renderer.text.into())
        } else {
            None
        };

        match val.contents {
            Contents::LiveChatTextMessageRenderer(r) => {
                let posted_at = Some(r.timestamp_usec.clone().into());

                let mut content = Content::new();
                content.add("TextMessageID", Some(r.id.clone()));

                vec![
                    SimpleChatEntity {
                        id: val.action_id,
                        author_external_channel_id: None,
                        posted_at,
                        category: CategoryValue::Banner,
                        author_name,
                        content,
                    },
                    SimpleChatEntity::from(r),
                ]
            }
            Contents::LiveChatBannerRedirectRenderer(r) => {
                let mut content = Content::new();
                content.add("BannerMessage", Some(r.banner_message.into()));
                content.add(
                    "RedirectVideoID",
                    Some(
                        r.inline_action_button
                            .button_renderer
                            .command
                            .watch_endpoint
                            .video_id
                            .clone(),
                    ),
                );

                vec![SimpleChatEntity {
                    id: val.action_id,
                    author_external_channel_id: None,
                    posted_at: None,
                    category: CategoryValue::Banner,
                    author_name,
                    content,
                }]
            }
        }
    }
}
