mod category;
pub(crate) mod content;
mod posted_at;

pub(crate) use category::CategoryValue;
pub(crate) use content::Content;
pub(crate) use posted_at::PostedAtValue;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SimpleChatEntity {
    pub id: String,
    pub author_external_channel_id: Option<String>,
    pub posted_at: Option<PostedAtValue>,
    pub category: CategoryValue,
    pub author_name: Option<String>,
    pub content: Content,
}
