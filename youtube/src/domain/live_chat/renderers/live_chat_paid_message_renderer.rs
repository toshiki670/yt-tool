use super::values::{
    accessibility::Accessibility, author_badge::AuthorBadges,
    context_menu_endpoint::ContextMenuEndpoint, creator_heart_button::CreatorHeartButton,
    reply_button::ReplyButton, text::Text, thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatPaidMessageRenderer {
    pub author_badges: Option<AuthorBadges>,
    pub author_external_channel_id: String,
    pub author_name_text_color: i64,
    pub author_name: Option<Text>,
    pub author_photo: Thumbnails,
    pub buy_button: Option<serde_json::Value>,
    pub body_background_color: i64,
    pub body_text_color: i64,
    pub context_menu_accessibility: Accessibility,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub creator_heart_button: CreatorHeartButton,
    pub header_background_color: i64,
    pub header_overlay_image: Option<Thumbnails>,
    pub header_text_color: i64,
    pub id: String,
    #[serde(rename = "isV2Style")]
    pub is_v2style: bool,
    pub lower_bumper: Option<serde_json::Value>,
    pub message: Option<Text>,
    pub pdg_like_button: Option<serde_json::Value>,
    pub pdg_purchased_novelty_logging_directives: Option<serde_json::Value>,
    pub purchase_amount_text: Text,
    pub reply_button: Option<ReplyButton>,
    pub text_input_background_color: i64,
    pub timestamp_color: i64,
    pub timestamp_text: Option<Text>,
    pub timestamp_usec: TimestampUsec,
    pub tracking_params: String,
}
