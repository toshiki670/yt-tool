use super::live_chat::LiveChatEntity;
use super::simple_chat::SimpleChatEntity;

pub trait ChatServiceRepository {
    async fn convert_from_lines(&self) -> anyhow::Result<()>;
    async fn convert_from_chunk(&self) -> anyhow::Result<()>;
}

pub trait FetchLiveChatRepository {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>>;
    fn first(&self) -> anyhow::Result<LiveChatEntity>;
}

pub trait SaveSimpleChatRepository {
    fn bulk_create(&self, chats: Vec<SimpleChatEntity>) -> anyhow::Result<()>;
}
