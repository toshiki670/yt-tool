use crate::domain::live_chat::LiveChatEntity;

pub trait FetchLiveChatRepository {
    fn all(&self) -> anyhow::Result<Vec<LiveChatEntity>>;
    fn one_chunk(&self) -> anyhow::Result<LiveChatEntity>;
}
