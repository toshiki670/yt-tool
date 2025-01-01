use super::simple_chat::SimpleChatEntity;

pub trait ChatRepository {
    fn all(&self) -> anyhow::Result<Vec<SimpleChatEntity>>;
    fn bulk_create(&self, chats: Vec<SimpleChatEntity>) -> anyhow::Result<()>;
}
