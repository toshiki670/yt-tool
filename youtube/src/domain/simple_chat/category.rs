use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

pub fn serialize_using_display<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: fmt::Display,
{
    serializer.serialize_str(&value.to_string())
}

impl fmt::Display for CategoryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CategoryValue::TextMessage => write!(f, "Text message"),
            CategoryValue::PaidMessage => write!(f, "Paid message"),
            CategoryValue::SponsorshipsGiftRedemptionAnnouncement => {
                write!(f, "Sponsorships gift redemption announcement")
            }
            CategoryValue::TickerPaidMessageItem => write!(f, "Ticker paid message item"),
            CategoryValue::ViewerEngagementMessage => {
                write!(f, "Viewer engagement message")
            }
            CategoryValue::SponsorshipsGiftPurchaseAnnouncement => {
                write!(f, "Sponsorships gift purchase announcement")
            }
            CategoryValue::MembershipItem => write!(f, "Membership item"),
        }
    }
}
