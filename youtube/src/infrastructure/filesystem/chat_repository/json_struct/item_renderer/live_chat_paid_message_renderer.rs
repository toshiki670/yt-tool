use super::{
    values::{
        accessibility::Accessibility, author_badge::AuthorBadges,
        context_menu_endpoint::ContextMenuEndpoint, creator_heart_button::CreatorHeartButton,
        message::Message, reply_button::ReplyButton, simple_text::SimpleText,
        thumbnails::Thumbnails, timestamp_usec::TimestampUsec,
    },
    CommonRenderer,
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

impl LiveChatPaidMessageRenderer {
    pub fn message_text(&self) -> String {
        if let Some(message) = &self.message {
            format!("{}: {}", self.purchase_amount_text.simple_text, message)
        } else {
            self.purchase_amount_text.simple_text.clone()
        }
    }
}

impl Into<CommonRenderer> for LiveChatPaidMessageRenderer {
    fn into(self) -> CommonRenderer {
        let message = self.message_text();

        let is_moderator = if let Some(author_badges) = &self.author_badges {
            author_badges.has_moderator()
        } else {
            false
        };

        let membership_months = if let Some(author_badges) = &self.author_badges {
            author_badges.fetch_membership_months()
        } else {
            None
        };
        let membership_months = membership_months.unwrap_or("".to_string());

        CommonRenderer {
            id: self.id,
            timestamp_usec: self.timestamp_usec.into(),
            author_external_channel_id: self.author_external_channel_id,
            author_name: self.author_name.into(),
            message: message,
            is_moderator,
            membership_months: membership_months,
        }
    }
}
