use super::SimpleChatEntity;

pub trait SaveSimpleChatRepository {
    fn bulk_create(&mut self, chats: Vec<SimpleChatEntity>) -> anyhow::Result<()>;
}
