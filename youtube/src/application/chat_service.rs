use crate::domain::repositories::ChatServiceRepository;
use futures::future;
use std::sync::Arc;

pub struct ChatConvertService<T: ChatServiceRepository> {
    chat_service_repositories: Vec<Arc<T>>,
}

impl<T: ChatServiceRepository> ChatConvertService<T> {
    pub fn new(chat_service_repositories: Vec<Arc<T>>) -> Self {
        Self {
            chat_service_repositories,
        }
    }

    pub fn move_chat_service_repository(self) -> Vec<Arc<T>> {
        self.chat_service_repositories
    }
}

impl<T> ChatConvertService<T>
where
    T: ChatServiceRepository,
{
    pub async fn convert_from_lines(&self) -> anyhow::Result<()> {
        let futures = self
            .chat_service_repositories
            .iter()
            .map(|chat_service| chat_service.convert_from_lines())
            .collect::<Vec<_>>();

        future::join_all(futures).await;

        Ok(())
    }

    pub async fn convert_from_chunk(&self) -> anyhow::Result<()> {
        let futures = self
            .chat_service_repositories
            .iter()
            .map(|chat_service| chat_service.convert_from_chunk())
            .collect::<Vec<_>>();

        future::join_all(futures).await;

        Ok(())
    }
}
