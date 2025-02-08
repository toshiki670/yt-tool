use crate::domain::{
    live_chat::renderers::poll_renderer::PollRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<Box<PollRenderer>> for SimpleChatEntity {
    fn from(val: Box<PollRenderer>) -> Self {
        let mut content = Content::new();
        content.add(
            "PollQuestion",
            Some(String::from(val.header.poll_header_renderer.poll_question)),
        );

        for (index, choice) in val.choices.into_iter().enumerate() {
            let content_text = if let Some(vote_percentage) = choice.vote_percentage {
                format!("{}({}%)", choice.text, vote_percentage)
            } else {
                choice.text.to_string()
            };
            content.add(format!("No. {}", index + 1).as_str(), Some(content_text));
        }

        SimpleChatEntity {
            id: val.live_chat_poll_id,
            author_external_channel_id: None,
            posted_at: None,
            category: CategoryValue::Poll,
            author_name: None,
            content,
        }
    }
}
