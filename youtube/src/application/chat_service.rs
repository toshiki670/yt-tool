use crate::domain::repositories::ChatRepository;

pub struct ChatConvertService {
    from_chat: Box<dyn ChatRepository>,
    to_chat: Box<dyn ChatRepository>,
}

impl ChatConvertService {
    pub fn new(from_chat: Box<dyn ChatRepository>, to_chat: Box<dyn ChatRepository>) -> Self {
        Self { from_chat, to_chat }
    }

    pub fn convert(&self) -> anyhow::Result<()> {
        let chats = self.from_chat.all()?;
        self.to_chat.bulk_create(chats)
    }
}
