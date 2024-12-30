// https://transform.tools/json-to-rust-serde
mod item_renderer;

use std::{
    fs::File,
    io::{BufRead as _, BufReader},
};

use crate::domain::chat::{CategoryValue, ChatEntity};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonStruct {
    pub is_live: bool,
    pub click_tracking_params: Option<String>,
    pub replay_chat_item_action: ReplayChatItemAction,
    pub video_offset_time_msec: Option<String>,
}

impl JsonStruct {
    pub fn try_into_chat_domains(self) -> anyhow::Result<Vec<ChatEntity>> {
        let mut results = Vec::new();

        for action in self.replay_chat_item_action.actions {
            let chat_domain: ChatEntity = action.try_into()?;
            results.push(chat_domain);
        }

        Ok(results)
    }

    pub fn all_from_file(file: &File) -> anyhow::Result<Vec<ChatEntity>> {
        let mut chats = Vec::new();

        for line in BufReader::new(file).lines() {
            let line = line?;
            let chat = serde_json::from_str::<JsonStruct>(&line)?;
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

impl TryInto<ChatEntity> for Action {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<ChatEntity> {
        let item = if let Some(add_chat_item_action) = self.add_chat_item_action {
            add_chat_item_action.item
        } else if let Some(add_live_chat_ticker_item_action) = self.add_live_chat_ticker_item_action
        {
            add_live_chat_ticker_item_action.item
        } else {
            return Err(JsonStructError::NotFoundItemForActionError.into());
        };

        let category: CategoryValue = match item {
            item_renderer::Item::LiveChatPaidMessageRenderer(_) => CategoryValue::ChatPaidMessage,
            item_renderer::Item::LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer(_) => {
                CategoryValue::ChatSponsorshipsGiftRedemptionAnnouncement
            }
            item_renderer::Item::LiveChatTextMessageRenderer(_) => CategoryValue::ChatTextMessage,
            item_renderer::Item::LiveChatTickerPaidMessageItemRenderer(_) => {
                CategoryValue::ChatTickerPaidMessageItem
            }
            item_renderer::Item::LiveChatViewerEngagementMessageRenderer(_) => {
                CategoryValue::ChatViewerEngagementMessage
            }
            item_renderer::Item::None => return Err(JsonStructError::InvalidCategoryItemError.into()),
        };

        let renderer: item_renderer::CommonRenderer = match item {
            item_renderer::Item::LiveChatPaidMessageRenderer(renderer) => renderer.into(),
            item_renderer::Item::LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer(
                renderer,
            ) => renderer.into(),
            item_renderer::Item::LiveChatTextMessageRenderer(renderer) => renderer.into(),
            item_renderer::Item::LiveChatTickerPaidMessageItemRenderer(renderer) => renderer.into(),
            item_renderer::Item::LiveChatViewerEngagementMessageRenderer(renderer) => {
                renderer.into()
            }
            item_renderer::Item::None => unreachable!(),
        };

        Ok(ChatEntity {
            posted_at: renderer.timestamp_usec.into(),
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


#[derive(thiserror::Error, Debug)]
pub enum JsonStructError {
    #[error("Not found item for action")]
    NotFoundItemForActionError,

    #[error("Invalid category item error")]
    InvalidCategoryItemError,
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

            let json_chat = serde_json::from_str::<JsonStruct>(&RAW_JSON)?;
            let chat_domains = json_chat.try_into_chat_domains()?;
            let actual = chat_domains.len();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_chat_text_message_category() -> anyhow::Result<()> {
            let expected = CategoryValue::ChatTextMessage;

            let json_chat = serde_json::from_str::<JsonStruct>(&RAW_JSON)?;
            let chat_domains = json_chat.try_into_chat_domains()?;
            let first = chat_domains.first().context("There is no chat")?;
            let actual = first.category.clone();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_timestamp_usec() -> anyhow::Result<()> {
            let expected = Utc.timestamp_micros(1733370114906095).unwrap();

            let json_chat = serde_json::from_str::<JsonStruct>(&RAW_JSON)?;
            let chat_domains = json_chat.try_into_chat_domains()?;
            let first = chat_domains.first().context("There is no chat")?;
            let actual: DateTime<Utc> = first.posted_at.into();

            assert_eq!(expected, actual);
            Ok(())
        }
    }

    mod live_chat_viewer_engagement_message_renderer {
        use super::*;
        use anyhow::Context as _;
        use chrono::prelude::*;

        const RAW_JSON: &str = r#"
            {"replayChatItemAction": {"actions": [{"clickTrackingParams": "CAEQl98BIhMI-ZrpttyPigMV0rpWAR2tPxON", "addChatItemAction": {"item": {"liveChatViewerEngagementMessageRenderer": {"id": "CjEKL0NPTU1VTklUWV9HVUlERUxJTkVTX1ZFTTIwMjQvMTIvMDQtMTk6NDg6NTIuNTA3", "timestampUsec": "1733370532507093", "icon": {"iconType": "YOUTUBE_ROUND"}, "message": {"runs": [{"text": "Welcome to live chat! Remember to guard your privacy and abide by our community guidelines."}]}, "actionButton": {"buttonRenderer": {"style": "STYLE_BLUE_TEXT", "size": "SIZE_DEFAULT", "isDisabled": false, "text": {"simpleText": "Learn more"}, "navigationEndpoint": {"clickTrackingParams": "CCIQ8FsiEwj5mum23I-KAxXSulYBHa0_E40=", "commandMetadata": {"webCommandMetadata": {"url": "//support.google.com/youtube/answer/2853856?hl=en#safe", "webPageType": "WEB_PAGE_TYPE_UNKNOWN", "rootVe": 83769}}, "urlEndpoint": {"url": "//support.google.com/youtube/answer/2853856?hl=en#safe", "target": "TARGET_NEW_WINDOW"}}, "trackingParams": "CCIQ8FsiEwj5mum23I-KAxXSulYBHa0_E40=", "accessibilityData": {"accessibilityData": {"label": "Learn more"}}}}, "trackingParams": "CAEQl98BIhMI-ZrpttyPigMV0rpWAR2tPxON"}}}}]}, "videoOffsetTimeMsec": "-87005", "isLive": true}
        "#;

        #[test]
        fn it_has_one_domain_chat() -> anyhow::Result<()> {
            let expected = 1;

            let json_chat = serde_json::from_str::<JsonStruct>(&RAW_JSON)?;
            let chat_domains = json_chat.try_into_chat_domains()?;
            let actual = chat_domains.len();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_chat_text_message_category() -> anyhow::Result<()> {
            let expected = CategoryValue::ChatViewerEngagementMessage;

            let json_chat = serde_json::from_str::<JsonStruct>(&RAW_JSON)?;
            let chat_domains = json_chat.try_into_chat_domains()?;
            let first = chat_domains.first().context("There is no chat")?;
            let actual = first.category.clone();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_timestamp_usec() -> anyhow::Result<()> {
            let expected = Utc.timestamp_micros(1733370114906095).unwrap();

            let json_chat = serde_json::from_str::<JsonStruct>(&RAW_JSON)?;
            let chat_domains = json_chat.try_into_chat_domains()?;
            let first = chat_domains.first().context("There is no chat")?;
            let actual: DateTime<Utc> = first.posted_at.into();

            assert_eq!(expected, actual);
            Ok(())
        }
    }
}
