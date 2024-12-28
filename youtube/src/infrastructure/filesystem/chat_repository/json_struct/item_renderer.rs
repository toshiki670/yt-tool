mod live_chat_paid_message_renderer;
mod live_chat_renderer;
mod live_chat_ticker_paid_message_item_renderer;
mod values;

use live_chat_paid_message_renderer::LiveChatPaidMessageRenderer;
use live_chat_renderer::LiveChatRenderer;
use live_chat_ticker_paid_message_item_renderer::LiveChatTickerPaidMessageItemRenderer;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Item {
    #[default]
    None,
    LiveChatPaidMessageRenderer(LiveChatPaidMessageRenderer),
    LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer(LiveChatRenderer),
    LiveChatTextMessageRenderer(LiveChatRenderer),
    LiveChatTickerPaidMessageItemRenderer(LiveChatTickerPaidMessageItemRenderer),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonRenderer {
    pub timestamp_usec: String,
    pub author_external_channel_id: String,
    pub author_name: String,
    pub message: String,
}