use crate::domain::{
    live_chat::item::renderers::live_chat_ticker_paid_message_item_renderer::LiveChatTickerPaidMessageItemRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl From<Box<LiveChatTickerPaidMessageItemRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatTickerPaidMessageItemRenderer>) -> Self {
        SimpleChatEntity {
            id: val.id,
            posted_at: val
                .show_item_endpoint
                .show_live_chat_item_endpoint
                .renderer
                .live_chat_paid_message_renderer
                .timestamp_usec
                .clone()
                .into(),
            author_external_channel_id: val.author_external_channel_id,
            author_name: val.author_username.into(),
            content: val
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
