use crate::domain::{
    live_chat::item::renderers::live_chat_ticker_paid_message_item_renderer::LiveChatTickerPaidMessageItemRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl Into<SimpleChatEntity> for LiveChatTickerPaidMessageItemRenderer {
    fn into(self) -> SimpleChatEntity {
        SimpleChatEntity {
            id: self.id,
            posted_at: self
                .show_item_endpoint
                .show_live_chat_item_endpoint
                .renderer
                .live_chat_paid_message_renderer
                .timestamp_usec
                .clone()
                .into(),
            author_external_channel_id: self.author_external_channel_id,
            author_name: self.author_username.into(),
            content: self
                .show_item_endpoint
                .show_live_chat_item_endpoint
                .renderer
                .live_chat_paid_message_renderer
                .message_text(),
            is_moderator: false,
            membership_months: "".to_string(),
            category: CategoryValue::ChatTickerPaidMessageItem,
        }
    }
}
