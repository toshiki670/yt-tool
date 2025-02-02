mod category;
pub(crate) mod content;
mod posted_at;

pub(crate) use category::CategoryValue;
pub(crate) use content::Content;
pub(crate) use posted_at::PostedAtValue;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleChatEntity {
    pub id: String,
    pub author_external_channel_id: String,
    pub posted_at: PostedAtValue,
    pub category: CategoryValue,
    pub author_name: String,
    pub content: Content,
}
