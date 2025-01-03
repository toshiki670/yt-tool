use super::SimpleChatEntity;

pub trait SaveSimpleChatRepository {
    fn bulk_create(&self, chats: Vec<SimpleChatEntity>) -> anyhow::Result<()>;
}
