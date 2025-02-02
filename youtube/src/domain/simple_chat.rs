mod category;
mod posted_at;

pub(crate) use category::{serialize_using_display, CategoryValue};
pub(crate) use posted_at::PostedAtValue;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleChatEntity {
    pub id: String,
    pub author_external_channel_id: String,
    pub posted_at: PostedAtValue,
    pub author_name: String,
    pub content: String,
    pub is_moderator: bool,
    pub membership_months: String,
    #[serde(serialize_with = "serialize_using_display")]
    pub category: CategoryValue,
}
