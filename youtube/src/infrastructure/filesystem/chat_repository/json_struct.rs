// https://transform.tools/json-to-rust-serde
mod item_renderer;

use crate::domain::chat::{Category, Chat as ChatDomain};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonChat {
    pub is_live: bool,
    pub click_tracking_params: Option<String>,
    pub replay_chat_item_action: ReplayChatItemAction,
    pub video_offset_time_msec: Option<String>,
}

impl JsonChat {
    pub fn try_into_chat_domains(self) -> anyhow::Result<Vec<ChatDomain>> {
        let mut results = Vec::new();

        for action in self.replay_chat_item_action.actions {
            let chat_domain: ChatDomain = action.try_into()?;
            results.push(chat_domain);
        }

        Ok(results)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayChatItemAction {
    pub actions: Vec<Action>,
    pub video_offset_time_msec: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub click_tracking_params: Option<String>,
    pub add_chat_item_action: Option<AddChatItemAction>,
    pub add_live_chat_ticker_item_action: Option<AddLiveChatTickerItemAction>,
}

impl TryInto<ChatDomain> for Action {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<ChatDomain> {
        let item = if let Some(add_chat_item_action) = self.add_chat_item_action {
            add_chat_item_action.item
        } else if let Some(add_live_chat_ticker_item_action) = self.add_live_chat_ticker_item_action
        {
            add_live_chat_ticker_item_action.item
        } else {
            return Err(anyhow::anyhow!("No item found for action"));
        };

        let category: Category = match item {
            item_renderer::Item::LiveChatPaidMessageRenderer(_) => Category::ChatPaidMessage,
            item_renderer::Item::LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer(_) => {
                Category::ChatSponsorshipsGiftRedemptionAnnouncement
            }
            item_renderer::Item::LiveChatTextMessageRenderer(_) => Category::ChatTextMessage,
            item_renderer::Item::LiveChatTickerPaidMessageItemRenderer(_) => {
                Category::ChatTickerPaidMessageItem
            }
            item_renderer::Item::None => return Err(anyhow::anyhow!("No item found")),
        };

        let renderer: item_renderer::CommonRenderer = match item {
            item_renderer::Item::LiveChatPaidMessageRenderer(renderer) => renderer.into(),
            item_renderer::Item::LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer(
                renderer,
            ) => renderer.into(),
            item_renderer::Item::LiveChatTextMessageRenderer(renderer) => renderer.into(),
            item_renderer::Item::LiveChatTickerPaidMessageItemRenderer(renderer) => renderer.into(),
            item_renderer::Item::None => unreachable!(),
        };

        Ok(ChatDomain {
            timestamp_usec: renderer.timestamp_usec,
            author_external_channel_id: renderer.author_external_channel_id,
            author_name: renderer.author_name,
            message: renderer.message,
            is_moderator: false,
            membership_months: "1".to_string(),
            category: category,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddChatItemAction {
    pub client_id: String,
    pub item: item_renderer::Item,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLiveChatTickerItemAction {
    pub item: item_renderer::Item,
    pub duration_sec: String,
}
