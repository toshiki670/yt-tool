pub(crate) mod renderers;
pub(crate) mod values;

use renderers::{
    live_chat_membership_item_renderer::LiveChatMembershipItemRenderer,
    live_chat_paid_message_renderer::LiveChatPaidMessageRenderer,
    live_chat_renderer::LiveChatRenderer,
    live_chat_sponsorships_gift_purchase_announcement_renderer::LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer,
    live_chat_ticker_paid_message_item_renderer::LiveChatTickerPaidMessageItemRenderer,
    live_chat_viewer_engagement_message_renderer::LiveChatViewerEngagementMessageRenderer,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Item {
    #[default]
    None,
    LiveChatPaidMessageRenderer(LiveChatPaidMessageRenderer),
    LiveChatPaidStickerRenderer(serde_json::Value),
    LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer(
        LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer,
    ),
    LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer(LiveChatRenderer),
    LiveChatTextMessageRenderer(LiveChatRenderer),
    LiveChatTickerPaidMessageItemRenderer(LiveChatTickerPaidMessageItemRenderer),
    LiveChatTickerSponsorItemRenderer(serde_json::Value),
    LiveChatViewerEngagementMessageRenderer(LiveChatViewerEngagementMessageRenderer),
    LiveChatMembershipItemRenderer(LiveChatMembershipItemRenderer),
}
