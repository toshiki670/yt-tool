use crate::domain::live_chat::LiveChatEntity;

pub trait FetchLiveChatRepository {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>>;
    fn first(&self) -> anyhow::Result<LiveChatEntity>;
}
