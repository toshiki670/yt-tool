// https://transform.tools/json-to-rust-serde
mod item_renderer;

use std::{
    fs::File,
    io::{BufRead as _, BufReader},
};

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

    pub fn all_from_file(file: &File) -> anyhow::Result<Vec<ChatDomain>> {
        let mut chats = Vec::new();

        for line in BufReader::new(file).lines() {
            let line = line?;
            let chat = serde_json::from_str::<JsonChat>(&line)?;
            let chat_domains = chat.try_into_chat_domains()?;
            chats.extend(chat_domains);
        }

        Ok(chats)
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

#[cfg(test)]
mod tests {

    use super::*;

    mod live_chat_text_message_renderer {
        use super::*;
        use anyhow::Context as _;
        use chrono::prelude::*;

        const RAW_JSON: &str = r#"
            {"replayChatItemAction":{"actions":[{"clickTrackingParams":"clickTrackingParams","addChatItemAction":{"item":{"liveChatTextMessageRenderer":{"message":{"runs":[{"text":"メッセージ"}]},"authorName":{"simpleText":"authorName"},"authorPhoto":{"thumbnails":[{"url":"https://yt4.ggpht.com/","width":32,"height":32},{"url":"https://yt4.ggpht.com/","width":64,"height":64}]},"contextMenuEndpoint":{"clickTrackingParams":"clickTrackingParams","commandMetadata":{"webCommandMetadata":{"ignoreNavigation":true}},"liveChatItemContextMenuEndpoint":{"params":"params=="}},"id":"id","timestampUsec":"1733370114906095","authorExternalChannelId":"authorExternalChannelId","contextMenuAccessibility":{"accessibilityData":{"label":"Chat actions"}},"trackingParams":"trackingParams"}},"clientId":"clientId"}}]},"videoOffsetTimeMsec":"-416809","isLive":true}
        "#;

        #[test]
        fn it_has_one_domain_chat() -> anyhow::Result<()> {
            let expected = 1;

            let json_chat = serde_json::from_str::<JsonChat>(&RAW_JSON)?;
            let chat_domains = json_chat.try_into_chat_domains()?;
            let actual = chat_domains.len();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_chat_text_message_category() -> anyhow::Result<()> {
            let expected = Category::ChatTextMessage;

            let json_chat = serde_json::from_str::<JsonChat>(&RAW_JSON)?;
            let chat_domains = json_chat.try_into_chat_domains()?;
            let first = chat_domains.first().context("There is no chat")?;
            let actual = first.category.clone();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_timestamp_usec() -> anyhow::Result<()> {
            let expected = Utc.timestamp_micros(1733370114906095).unwrap();

            let json_chat = serde_json::from_str::<JsonChat>(&RAW_JSON)?;
            let chat_domains = json_chat.try_into_chat_domains()?;
            let first = chat_domains.first().context("There is no chat")?;
            let actual = first.timestamp_usec;

            assert_eq!(expected, actual);
            Ok(())
        }
    }
}
