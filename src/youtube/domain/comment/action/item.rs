use serde::{Deserialize, Serialize};

mod properties;

use properties::{
    accessibility::Accessibility, author_badge::AuthorBadge, author_name::AuthorName,
    context_menu_endpoint::ContextMenuEndpoint, message::Message, thumbnail::Thumbnails,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub live_chat_sponsorships_gift_redemption_announcement_renderer:
        Option<LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer>,
    pub live_chat_text_message_renderer: Option<LiveChatTextMessageRenderer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer {
    pub author_external_channel_id: String,
    pub author_name: AuthorName,
    pub author_photo: Thumbnails,
    pub context_menu_accessibility: Accessibility,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub id: String,
    pub message: Message,
    pub timestamp_usec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTextMessageRenderer {
    pub author_badges: Option<Vec<AuthorBadge>>,
    pub author_external_channel_id: String,
    pub author_name: AuthorName,
    pub author_photo: Thumbnails,
    pub context_menu_accessibility: Accessibility,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub id: String,
    pub message: Message,
    pub timestamp_usec: String,
}
