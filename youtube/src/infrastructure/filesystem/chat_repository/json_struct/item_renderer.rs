mod live_chat_paid_message_renderer;
mod live_chat_renderer;
mod live_chat_ticker_paid_message_item_renderer;
mod live_chat_viewer_engagement_message_renderer;
mod live_chat_paid_sticker_renderer;
mod live_chat_sponsorships_gift_purchase_announcement_renderer;
mod values;

use chrono::prelude::*;
use live_chat_paid_message_renderer::LiveChatPaidMessageRenderer;
use live_chat_renderer::LiveChatRenderer;
use live_chat_ticker_paid_message_item_renderer::LiveChatTickerPaidMessageItemRenderer;
use live_chat_viewer_engagement_message_renderer::LiveChatViewerEngagementMessageRenderer;
use live_chat_paid_sticker_renderer::LiveChatPaidStickerRenderer;
use live_chat_sponsorships_gift_purchase_announcement_renderer::LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer;
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
    LiveChatViewerEngagementMessageRenderer(LiveChatViewerEngagementMessageRenderer),
    LiveChatPaidStickerRenderer(LiveChatPaidStickerRenderer),
    LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer(LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonRenderer {
    pub timestamp_usec: DateTime<Utc>,
    pub author_external_channel_id: String,
    pub author_name: String,
    pub message: String,
}
