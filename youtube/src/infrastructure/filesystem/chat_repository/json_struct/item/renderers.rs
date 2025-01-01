pub(super) mod live_chat_membership_item_renderer;
pub(super) mod live_chat_paid_message_renderer;
pub(super) mod live_chat_renderer;
pub(super) mod live_chat_sponsorships_gift_purchase_announcement_renderer;
pub(super) mod live_chat_ticker_paid_message_item_renderer;
pub(super) mod live_chat_viewer_engagement_message_renderer;

use super::values;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonRenderer {
    pub id: String,
    pub timestamp_usec: DateTime<Utc>,
    pub author_external_channel_id: String,
    pub author_name: String,
    pub message: String,
    pub is_moderator: bool,
    pub membership_months: String,
}
