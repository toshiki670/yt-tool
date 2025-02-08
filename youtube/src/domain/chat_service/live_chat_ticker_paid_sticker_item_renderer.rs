use crate::domain::{
    live_chat::item::renderers::live_chat_ticker_paid_sticker_item_renderer::LiveChatTickerPaidStickerItemRenderer,
    simple_chat::{CategoryValue, Content, SimpleChatEntity},
};

impl From<Box<LiveChatTickerPaidStickerItemRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatTickerPaidStickerItemRenderer>) -> Self {
        let live_chat_paid_sticker_renderer = val
            .show_item_endpoint
            .show_live_chat_item_endpoint
            .renderer
            .live_chat_paid_sticker_renderer;

        let posted_at = Some(
            live_chat_paid_sticker_renderer
                .timestamp_usec
                .clone()
                .into(),
        );

        let author_name = Some(live_chat_paid_sticker_renderer.author_name.clone().into());

        let purchase_amount_text = live_chat_paid_sticker_renderer
            .purchase_amount_text
            .simple_text;

        let mut content = Content::new();
        content.add("purchaseAmountText", Some(purchase_amount_text));

        SimpleChatEntity {
            id: val.id,
            author_external_channel_id: Some(val.author_external_channel_id),
            category: CategoryValue::TickerPaidMessageItem,
            posted_at,
            author_name,
            content,
        }
    }
}
