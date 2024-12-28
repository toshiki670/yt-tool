use super::{
    values::{
        accessibility::Accessibility, author_badge::AuthorBadge,
        context_menu_endpoint::ContextMenuEndpoint, creator_heart_button::CreatorHeartButton,
        message::Message, reply_button::ReplyButton, simple_text::SimpleText,
        thumbnails::Thumbnails,
    },
    CommonRenderer,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatPaidMessageRenderer {
    pub id: String,
    pub timestamp_usec: String,
    pub author_name: SimpleText,
    pub author_photo: Thumbnails,
    pub purchase_amount_text: SimpleText,
    pub message: Message,
    pub header_background_color: i64,
    pub header_text_color: i64,
    pub body_background_color: i64,
    pub body_text_color: i64,
    pub author_external_channel_id: String,
    pub author_name_text_color: i64,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub timestamp_color: i64,
    pub context_menu_accessibility: Accessibility,
    pub timestamp_text: SimpleText,
    pub tracking_params: String,
    pub author_badges: Vec<AuthorBadge>,
    pub text_input_background_color: i64,
    pub creator_heart_button: CreatorHeartButton,
    #[serde(rename = "isV2Style")]
    pub is_v2style: bool,
    pub reply_button: ReplyButton,
}

impl Into<CommonRenderer> for LiveChatPaidMessageRenderer {
    fn into(self) -> CommonRenderer {
        CommonRenderer {
            timestamp_usec: self.timestamp_usec,
            author_external_channel_id: self.author_external_channel_id,
            author_name: self.author_name.into(),
            message: self.message.into(),
        }
    }
}