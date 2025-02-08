use crate::domain::{
    live_chat::renderers::live_chat_ticker_paid_message_item_renderer::LiveChatTickerPaidMessageItemRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<Box<LiveChatTickerPaidMessageItemRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatTickerPaidMessageItemRenderer>) -> Self {
        let live_chat_paid_message_renderer = val
            .show_item_endpoint
            .show_live_chat_item_endpoint
            .renderer
            .live_chat_paid_message_renderer;

        let posted_at = Some(
            live_chat_paid_message_renderer
                .timestamp_usec
                .clone()
                .into(),
        );

        let author_name = Some(val.author_username.into());

        let mut content = Content::new();
        if let Some(message) = live_chat_paid_message_renderer.message {
            content.add("message", Some(message.into()));
        }

        content.add(
            "purchaseAmount",
            Some(
                live_chat_paid_message_renderer
                    .purchase_amount_text
                    .clone()
                    .into(),
            ),
        );

        SimpleChatEntity {
            id: val.id,
            author_external_channel_id: Some(val.author_external_channel_id),
            posted_at,
            category: CategoryValue::TickerPaidMessageItem,
            author_name,
            content,
        }
    }
}
