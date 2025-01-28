use crate::domain::{
    live_chat::item::renderers::live_chat_ticker_paid_sticker_item_renderer::LiveChatTickerPaidStickerItemRenderer,
    simple_chat::{CategoryValue, SimpleChatEntity},
};

impl From<Box<LiveChatTickerPaidStickerItemRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatTickerPaidStickerItemRenderer>) -> Self {
        let live_chat_paid_sticker_renderer = val
            .show_item_endpoint
            .show_live_chat_item_endpoint
            .renderer
            .live_chat_paid_sticker_renderer;
        SimpleChatEntity {
            id: val.id,
            posted_at: live_chat_paid_sticker_renderer
                .timestamp_usec
                .clone()
                .into(),
            author_external_channel_id: val.author_external_channel_id,
            author_name: live_chat_paid_sticker_renderer.author_name.clone().into(),
            content: live_chat_paid_sticker_renderer.purchase_amount_text.simple_text,
            is_moderator: false,
            membership_months: "".to_string(),
            category: CategoryValue::ChatTickerPaidMessageItem,
        }
    }
}
