mod category;
mod posted_at;

pub(crate) use category::CategoryValue;
pub(crate) use posted_at::PostedAtValue;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleChatEntity {
    pub id: String,
    pub author_external_channel_id: String,
    pub author_name: String,
    pub category: CategoryValue,
    pub is_moderator: bool,
    pub membership_months: String,
    pub content: String,
    pub posted_at: PostedAtValue,
}
