use super::chat::Chat;



// pub trait Repositories {
//     fn chat_repository(&self) -> Box<dyn ChatRepository>;
// }

pub trait ChatRepository {
    fn all(&self) -> Vec<Chat>;
    fn bulk_create(&self, chats: Vec<Chat>);
}


