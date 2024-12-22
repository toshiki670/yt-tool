

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Chat {
    pub timestamp_usec: String,
    pub author_external_channel_id: String,
    pub author_name: String,
    pub message: String,
    pub is_moderator: bool,
    pub membership_months: String,
    pub category: Category,
}


#[derive(Default, Debug, Clone, PartialEq)]
pub enum Category {
    #[default]
    ChatTextMessage,
    ChatSponsorshipsGiftRedemptionAnnouncement,
}


