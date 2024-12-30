#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum CategoryValue {
    #[default]
    ChatTextMessage,
    ChatPaidMessage,
    ChatSponsorshipsGiftRedemptionAnnouncement,
    ChatTickerPaidMessageItem,
}
