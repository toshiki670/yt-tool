use super::live_chat::LiveChatEntity;
use super::simple_chat::SimpleChatEntity;

pub trait ChatServiceRepository {
    fn from_chat_repository(&self) -> &dyn FetchLiveChatRepository;
    fn to_chat_repository(&self) -> &dyn SaveSimpleChatRepository;
}

pub trait FetchLiveChatRepository {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>>;
    fn first(&self) -> anyhow::Result<LiveChatEntity>;
}

pub trait SaveSimpleChatRepository {
    fn bulk_create(&self, chats: Vec<SimpleChatEntity>) -> anyhow::Result<()>;
}
