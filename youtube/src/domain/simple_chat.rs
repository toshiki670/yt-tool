mod category;
mod content;
mod posted_at;

pub use category::CategoryValue;
pub use content::Content;
pub use posted_at::PostedAtValue;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleChatEntity {
    pub id: String,
    pub author_external_channel_id: Option<String>,
    pub posted_at: Option<PostedAtValue>,
    pub category: CategoryValue,
    pub author_name: Option<String>,
    pub content: Content,
}
