use super::chat_entity::ChatEntity;

pub trait ChatRepository {
    fn all(&self) -> anyhow::Result<Vec<ChatEntity>>;
    fn bulk_create(&self, chats: Vec<ChatEntity>) -> anyhow::Result<()>;
}
