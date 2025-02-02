use serde::{Serialize, Serializer};
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum CategoryValue {
    #[default]
    TextMessage,
    PaidMessage,
    SponsorshipsGiftPurchaseAnnouncement,
    SponsorshipsGiftRedemptionAnnouncement,
    TickerPaidMessageItem,
    ViewerEngagementMessage,
    MembershipItem,
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
            CategoryValue::TextMessage => "Text message",
            CategoryValue::PaidMessage => "Paid message",
            CategoryValue::SponsorshipsGiftRedemptionAnnouncement => {
                "Sponsorships gift redemption announcement"
            }
            CategoryValue::TickerPaidMessageItem => "Ticker paid message item",
            CategoryValue::ViewerEngagementMessage => "Viewer engagement message",
            CategoryValue::SponsorshipsGiftPurchaseAnnouncement => {
                "Sponsorships gift purchase announcement"
            }
            CategoryValue::MembershipItem => "Membership item",
        };

        write!(f, "{}", msg)
    }
}
