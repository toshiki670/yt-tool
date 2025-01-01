use std::fmt;

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum CategoryValue {
    #[default]
    ChatTextMessage,
    ChatPaidMessage,
    ChatSponsorshipsGiftPurchaseAnnouncement,
    ChatSponsorshipsGiftRedemptionAnnouncement,
    ChatTickerPaidMessageItem,
    ChatViewerEngagementMessage,
}

impl fmt::Display for CategoryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CategoryValue::ChatTextMessage => write!(f, "Chat text message"),
            CategoryValue::ChatPaidMessage => write!(f, "Chat paid message"),
            CategoryValue::ChatSponsorshipsGiftRedemptionAnnouncement => {
                write!(f, "Chat sponsorships gift redemption announcement")
            }
            CategoryValue::ChatTickerPaidMessageItem => write!(f, "Chat ticker paid message item"),
            CategoryValue::ChatViewerEngagementMessage => {
                write!(f, "Chat viewer engagement message")
            }
            CategoryValue::ChatSponsorshipsGiftPurchaseAnnouncement => {
                write!(f, "Chat sponsorships gift purchase announcement")
            }
        }
    }
}
