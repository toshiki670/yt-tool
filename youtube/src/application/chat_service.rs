use crate::domain::{
    live_chat::repository::FetchLiveChatRepository,
    simple_chat::{repository::SaveSimpleChatRepository, SimpleChatEntity},
};
use anyhow::Context;
use std::convert::TryInto;

pub struct ChatConvertService {
    from_chat: Box<dyn FetchLiveChatRepository>,
    to_chat: Box<dyn SaveSimpleChatRepository>,
}

impl ChatConvertService {
    pub fn new(
        from_chat: Box<dyn FetchLiveChatRepository>,
        to_chat: Box<dyn SaveSimpleChatRepository>,
    ) -> Self {
        Self { from_chat, to_chat }
    }

    pub fn convert(&self) -> anyhow::Result<()> {
        let live_chats = self.from_chat.all()?;
        let mut simple_chats = Vec::new();

        for (line_number, live_chat) in live_chats.into_iter().enumerate() {
            let chats: Vec<SimpleChatEntity> = live_chat.try_into().with_context(|| {
                format!("Failed to convert live chat at line {}", line_number + 1)
            })?;
            simple_chats.extend(chats);
        }

        self.to_chat.bulk_create(simple_chats)
    }
}
