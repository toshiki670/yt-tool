use crate::domain::{repositories::ChatServiceRepository, simple_chat::SimpleChatEntity};
use anyhow::Context;
use std::convert::TryInto;

pub struct ChatConvertService<'a, T: ChatServiceRepository> {
    chat_service_repository: &'a [T],
}

impl<'a, T: ChatServiceRepository> ChatConvertService<'a, T> {
    pub fn new(chat_service_repository: &'a [T]) -> Self {
        Self {
            chat_service_repository,
        }
    }
}

impl<'a, T: ChatServiceRepository> ChatConvertService<'a, T> {
    pub fn convert_from_lines(&mut self) -> anyhow::Result<()> {
        for chat_service in self.chat_service_repository {
            let from_chats = chat_service.from_chat_repository().all()?;
            let mut to_chats = Vec::new();

            for (line_number, from_chat) in from_chats.into_iter().enumerate() {
                let chats: Vec<SimpleChatEntity> = from_chat.try_into().with_context(|| {
                    format!("Failed to convert live chat at line {}", line_number + 1)
                })?;
                to_chats.extend(chats);
            }

            chat_service.to_chat_repository().bulk_create(to_chats)?;
        }

        Ok(())
    }

    pub fn convert_from_chunk(&mut self) -> anyhow::Result<()> {
        for chat_service in self.chat_service_repository {
            let live_chat = chat_service.from_chat_repository().first()?;
            let simple_chat = live_chat.try_into()?;
            chat_service.to_chat_repository().bulk_create(simple_chat)?;
        }

        Ok(())
    }
}
