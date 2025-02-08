use serde::{Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CategoryValue {
    Banner,
    MembershipItem,
    PaidMessage,
    Poll,
    SponsorshipsGiftPurchaseAnnouncement,
    SponsorshipsGiftRedemptionAnnouncement,
    TextMessage,
    TickerPaidMessageItem,
    UpdatedPoll,
    ViewerEngagementMessage,
}

impl Serialize for CategoryValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl fmt::Display for CategoryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            CategoryValue::Banner => "Banner",
            CategoryValue::MembershipItem => "Membership item",
            CategoryValue::PaidMessage => "Paid message",
            CategoryValue::Poll => "Pool",
            CategoryValue::SponsorshipsGiftPurchaseAnnouncement => {
                "Sponsorships gift purchase announcement"
            }
            CategoryValue::SponsorshipsGiftRedemptionAnnouncement => {
                "Sponsorships gift redemption announcement"
            }
            CategoryValue::TextMessage => "Text message",
            CategoryValue::TickerPaidMessageItem => "Ticker paid message item",
            CategoryValue::UpdatedPoll => "Updated pool",
            CategoryValue::ViewerEngagementMessage => "Viewer engagement message",
        };

        write!(f, "{}", msg)
    }
}
