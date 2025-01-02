use super::{live_chat::LiveChatEntity, simple_chat::SimpleChatEntity};

pub trait LiveChatRepository {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>>;
}

pub trait SimpleChatRepository {
    fn bulk_create(&self, chats: Vec<SimpleChatEntity>) -> anyhow::Result<()>;
}
