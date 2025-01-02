use super::values::{
    accessibility::Accessibility, author_badge::AuthorBadges,
    context_menu_endpoint::ContextMenuEndpoint, creator_heart_button::CreatorHeartButton,
    message::Message, reply_button::ReplyButton, simple_text::SimpleText, thumbnails::Thumbnails,
    timestamp_usec::TimestampUsec,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatPaidMessageRenderer {
    pub author_badges: Option<AuthorBadges>,
    pub author_external_channel_id: String,
    pub author_name_text_color: i64,
    pub author_name: SimpleText,
    pub author_photo: Thumbnails,
    pub body_background_color: i64,
    pub body_text_color: i64,
    pub context_menu_accessibility: Accessibility,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub creator_heart_button: CreatorHeartButton,
    pub header_background_color: i64,
    pub header_text_color: i64,
    pub id: String,
    #[serde(rename = "isV2Style")]
    pub is_v2style: bool,
    pub message: Option<Message>,
    pub purchase_amount_text: SimpleText,
    pub reply_button: Option<ReplyButton>,
    pub text_input_background_color: i64,
    pub timestamp_color: i64,
    pub timestamp_text: Option<SimpleText>,
    pub timestamp_usec: TimestampUsec,
    pub tracking_params: String,
}
