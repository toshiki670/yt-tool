use super::chat::Chat;

pub trait ChatRepository {
    fn all(&self) -> anyhow::Result<Vec<Chat>>;
    fn bulk_create(&self, chats: Vec<Chat>) -> anyhow::Result<()>;
}
