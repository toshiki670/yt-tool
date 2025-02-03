pub(crate) mod renderers;
pub(crate) mod values;

use renderers::{
    live_chat_membership_item_renderer::LiveChatMembershipItemRenderer,
    live_chat_paid_message_renderer::LiveChatPaidMessageRenderer,
    live_chat_sponsorships_gift_purchase_announcement_renderer::LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer,
    live_chat_sponsorships_gift_redemption_announcement_renderer::LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer,
    live_chat_text_message_renderer::LiveChatTextMessageRenderer,
    live_chat_ticker_paid_message_item_renderer::LiveChatTickerPaidMessageItemRenderer,
    live_chat_ticker_paid_sticker_item_renderer::LiveChatTickerPaidStickerItemRenderer,
    live_chat_viewer_engagement_message_renderer::LiveChatViewerEngagementMessageRenderer,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Item {
    LiveChatMembershipItemRenderer(Box<LiveChatMembershipItemRenderer>),
    LiveChatPaidMessageRenderer(Box<LiveChatPaidMessageRenderer>),
    LiveChatPaidStickerRenderer(serde_json::Value),
    LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer(
        Box<LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer>,
    ),
    LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer(
        Box<LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer>,
    ),
    LiveChatTextMessageRenderer(Box<LiveChatTextMessageRenderer>),
    LiveChatTickerPaidMessageItemRenderer(Box<LiveChatTickerPaidMessageItemRenderer>),
    LiveChatTickerPaidStickerItemRenderer(Box<LiveChatTickerPaidStickerItemRenderer>),
    LiveChatTickerSponsorItemRenderer(serde_json::Value),
    LiveChatViewerEngagementMessageRenderer(Box<LiveChatViewerEngagementMessageRenderer>),
}
