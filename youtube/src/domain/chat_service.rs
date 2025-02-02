pub(super) mod live_chat_membership_item_renderer;
pub(super) mod live_chat_paid_message_renderer;
pub(super) mod live_chat_sponsorships_gift_purchase_announcement_renderer;
pub(super) mod live_chat_sponsorships_gift_redemption_announcement_renderer;
pub(super) mod live_chat_text_message_renderer;
pub(super) mod live_chat_ticker_paid_message_item_renderer;
pub(super) mod live_chat_ticker_paid_sticker_item_renderer;
pub(super) mod live_chat_viewer_engagement_message_renderer;

use anyhow::bail;
use chrono::{DateTime, Utc};

use super::{
    live_chat::{
        item::{values::timestamp_usec::TimestampUsec, Item},
        Action, LiveChatEntity,
    },
    simple_chat::{PostedAtValue, SimpleChatEntity},
};

impl LiveChatEntity {
    pub async fn try_into_simple_chats(self) -> anyhow::Result<Vec<SimpleChatEntity>> {
        self.try_into()
    }
}

impl TryInto<Vec<SimpleChatEntity>> for LiveChatEntity {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Vec<SimpleChatEntity>> {
        let mut simple_chats = Vec::new();

        for action in self.replay_chat_item_action.actions {
            if let Some(simple_chat) = action.try_into()? {
                simple_chats.push(simple_chat);
            }
        }

        Ok(simple_chats)
    }
}

impl TryInto<Option<SimpleChatEntity>> for Action {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Option<SimpleChatEntity>> {
        let item = if let Some(add_chat_item_action) = self.add_chat_item_action {
            add_chat_item_action.item
        } else if let Some(add_live_chat_ticker_item_action) = self.add_live_chat_ticker_item_action
        {
            add_live_chat_ticker_item_action.item
        } else if self.live_chat_report_moderation_state_command.is_some()
            || self.remove_chat_item_action.is_some()
        {
            return Ok(None);
        } else {
            unreachable!();
        };

        item.try_into()
    }
}

impl TryInto<Option<SimpleChatEntity>> for Item {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Option<SimpleChatEntity>> {
        let renderer: SimpleChatEntity = match self {
            Item::None => bail!("no exists renderers"),
            Item::LiveChatMembershipItemRenderer(r) => r.into(),
            Item::LiveChatPaidMessageRenderer(r) => r.into(),
            Item::LiveChatPaidStickerRenderer(_) => return Ok(None),
            Item::LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer(r) => r.into(),
            Item::LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer(r) => r.into(),
            Item::LiveChatTextMessageRenderer(r) => r.into(),
            Item::LiveChatTickerPaidMessageItemRenderer(r) => r.into(),
            Item::LiveChatTickerSponsorItemRenderer(_) => return Ok(None),
            Item::LiveChatViewerEngagementMessageRenderer(r) => r.into(),
            Item::LiveChatTickerPaidStickerItemRenderer(r) => r.into(),
        };

        Ok(Some(renderer))
    }
}

impl From<TimestampUsec> for PostedAtValue {
    fn from(value: TimestampUsec) -> Self {
        Self::from(Into::<DateTime<Utc>>::into(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod live_chat_text_message_renderer {
        use crate::domain::simple_chat::CategoryValue;

        use super::*;
        use anyhow::Context as _;
        use chrono::prelude::*;
        use pretty_assertions::assert_eq;

        const RAW_JSON: &str = r#"
            {"replayChatItemAction":{"actions":[{"clickTrackingParams":"clickTrackingParams","addChatItemAction":{"item":{"liveChatTextMessageRenderer":{"message":{"runs":[{"text":"メッセージ"}]},"authorName":{"simpleText":"authorName"},"authorPhoto":{"thumbnails":[{"url":"https://yt4.ggpht.com/","width":32,"height":32},{"url":"https://yt4.ggpht.com/","width":64,"height":64}]},"contextMenuEndpoint":{"clickTrackingParams":"clickTrackingParams","commandMetadata":{"webCommandMetadata":{"ignoreNavigation":true}},"liveChatItemContextMenuEndpoint":{"params":"params=="}},"id":"id","timestampUsec":"1733370114906095","authorExternalChannelId":"authorExternalChannelId","contextMenuAccessibility":{"accessibilityData":{"label":"Chat actions"}},"trackingParams":"trackingParams"}},"clientId":"clientId"}}]},"videoOffsetTimeMsec":"-416809","isLive":true}
        "#;

        #[test]
        fn it_has_one_simple_chat() -> anyhow::Result<()> {
            let expected = 1;

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let actual = simple_chats.len();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_chat_text_message_category() -> anyhow::Result<()> {
            let expected = CategoryValue::ChatTextMessage;

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let first = simple_chats.first().context("There is no chat")?;
            let actual = first.category.clone();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_timestamp_usec() -> anyhow::Result<()> {
            let expected = Utc.timestamp_micros(1733370114906095).unwrap();

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let first = simple_chats.first().context("There is no chat")?;
            let actual: DateTime<Utc> = first.posted_at.into();

            assert_eq!(expected, actual);
            Ok(())
        }
    }

    mod live_chat_viewer_engagement_message_renderer {
        use crate::domain::simple_chat::CategoryValue;

        use super::*;
        use anyhow::Context as _;
        use chrono::prelude::*;
        use pretty_assertions::assert_eq;

        const RAW_JSON: &str = r#"
            {"replayChatItemAction": {"actions": [{"clickTrackingParams": "CAEQl98BIhMI-ZrpttyPigMV0rpWAR2tPxON", "addChatItemAction": {"item": {"liveChatViewerEngagementMessageRenderer": {"id": "CjEKL0NPTU1VTklUWV9HVUlERUxJTkVTX1ZFTTIwMjQvMTIvMDQtMTk6NDg6NTIuNTA3", "timestampUsec": "1733370532507093", "icon": {"iconType": "YOUTUBE_ROUND"}, "message": {"runs": [{"text": "Welcome to live chat! Remember to guard your privacy and abide by our community guidelines."}]}, "actionButton": {"buttonRenderer": {"style": "STYLE_BLUE_TEXT", "size": "SIZE_DEFAULT", "isDisabled": false, "text": {"simpleText": "Learn more"}, "navigationEndpoint": {"clickTrackingParams": "CCIQ8FsiEwj5mum23I-KAxXSulYBHa0_E40=", "commandMetadata": {"webCommandMetadata": {"url": "//support.google.com/youtube/answer/2853856?hl=en#safe", "webPageType": "WEB_PAGE_TYPE_UNKNOWN", "rootVe": 83769}}, "urlEndpoint": {"url": "//support.google.com/youtube/answer/2853856?hl=en#safe", "target": "TARGET_NEW_WINDOW"}}, "trackingParams": "CCIQ8FsiEwj5mum23I-KAxXSulYBHa0_E40=", "accessibilityData": {"accessibilityData": {"label": "Learn more"}}}}, "trackingParams": "CAEQl98BIhMI-ZrpttyPigMV0rpWAR2tPxON"}}}}]}, "videoOffsetTimeMsec": "-87005", "isLive": true}
        "#;

        #[test]
        fn it_has_one_simple_chat() -> anyhow::Result<()> {
            let expected = 1;

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let actual = simple_chats.len();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_chat_text_message_category() -> anyhow::Result<()> {
            let expected = CategoryValue::ChatViewerEngagementMessage;

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let first = simple_chats.first().context("There is no chat")?;
            let actual = first.category.clone();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_timestamp_usec() -> anyhow::Result<()> {
            let expected = Utc.timestamp_micros(1733370532507093).unwrap();

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let first = simple_chats.first().context("There is no chat")?;
            let actual: DateTime<Utc> = first.posted_at.into();

            assert_eq!(expected, actual);
            Ok(())
        }
    }

    mod live_chat_sponsorships_gift_purchase_announcement_renderer {
        use anyhow::Context as _;

        use super::*;
        use crate::domain::simple_chat::CategoryValue;
        use pretty_assertions::assert_eq;

        const RAW_JSON: &str = r#"
          {"replayChatItemAction": {"actions": [{"clickTrackingParams": "CAEQl98BIhMIh5KnseuPigMVhE31BR2YaRHr", "addChatItemAction": {"item": {"liveChatSponsorshipsGiftPurchaseAnnouncementRenderer": {"id": "ChwKGkNOX3NqcWZyajRvREZSM0J3Z1FkSVRZZDd3", "timestampUsec": "1733374539514652", "authorExternalChannelId": "UC0Fruh1Rn-PPpb8lrkww_ew", "header": {"liveChatSponsorshipsHeaderRenderer": {"authorName": {"simpleText": "村田美夏（ウルフ村田）"}, "authorPhoto": {"thumbnails": [{"url": "https://yt4.ggpht.com/ytc/AIdro_ngUufTAybBxOIjcibAvd8VCzex89HBlP-Dyo9fg-DxdyA=s32-c-k-c0x00ffffff-no-rj", "width": 32, "height": 32}, {"url": "https://yt4.ggpht.com/ytc/AIdro_ngUufTAybBxOIjcibAvd8VCzex89HBlP-Dyo9fg-DxdyA=s64-c-k-c0x00ffffff-no-rj", "width": 64, "height": 64}]}, "primaryText": {"runs": [{"text": "Gifted ", "bold": true}, {"text": "50", "bold": true}, {"text": " ", "bold": true}, {"text": "ホロライブ愛好会ch【旧:ホロライブ切り抜き hololive clips】", "bold": true}, {"text": " memberships", "bold": true}]}, "contextMenuEndpoint": {"clickTrackingParams": "CAkQ3MMKIhMIh5KnseuPigMVhE31BR2YaRHr", "commandMetadata": {"webCommandMetadata": {"ignoreNavigation": true}}, "liveChatItemContextMenuEndpoint": {"params": "Q2g0S0hBb2FRMDVmYzJweFpuSnFORzlFUmxJelFuZG5VV1JKVkZsa04zY2FLU29uQ2hoVlF6WnZURGhuWlV0TlpYbG5iUzAwT1dsTWNuRmxabmNTQzJkWWNrUkthakZyWjB0SklBRW9CRElhQ2hoVlF6QkdjblZvTVZKdUxWQlFjR0k0YkhKcmQzZGZaWGM0QWtnQVVDUSUzRA=="}}, "contextMenuAccessibility": {"accessibilityData": {"label": "Chat actions"}}, "image": {"thumbnails": [{"url": "https://www.gstatic.com/youtube/img/sponsorships/sponsorships_gift_purchase_announcement_artwork.png"}]}}}}}, "clientId": "CN_sjqfrj4oDFR3BwgQdITYd7w"}}]}, "videoOffsetTimeMsec": "4002297", "isLive": true}
        "#;

        #[test]
        fn it_has_one_simple_chat() -> anyhow::Result<()> {
            let expected = 1;

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let actual = simple_chats.len();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_chat_text_message_category() -> anyhow::Result<()> {
            let expected = CategoryValue::ChatSponsorshipsGiftPurchaseAnnouncement;

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let first = simple_chats.first().context("There is no chat")?;
            let actual = first.category.clone();

            assert_eq!(expected, actual);
            Ok(())
        }
    }
    mod live_chat_paid_message_renderer {
        use anyhow::Context as _;

        use super::*;
        use crate::domain::simple_chat::CategoryValue;
        use pretty_assertions::assert_eq;

        const RAW_JSON: &str = r#"
           {"replayChatItemAction": {"actions": [{"clickTrackingParams": "CAEQl98BIhMIg6vejuiPigMVXmr1BR0z6x8E", "addChatItemAction": {"item": {"liveChatPaidMessageRenderer": {"id": "ChwKGkNPZURnSVhvajRvREZjN0N3Z1FkV3hNMElR", "timestampUsec": "1733373660562877", "authorName": {"simpleText": "にねんせい"}, "authorPhoto": {"thumbnails": [{"url": "https://yt4.ggpht.com/QHs0vgTB8C4DpCe7cC5YZMiJYdTK_NqbfxZuGwbdy4os29j3T0Tk9Qzm04yrrNL0TYNaS6QVKg=s32-c-k-c0x00ffffff-no-rj", "width": 32, "height": 32}, {"url": "https://yt4.ggpht.com/QHs0vgTB8C4DpCe7cC5YZMiJYdTK_NqbfxZuGwbdy4os29j3T0Tk9Qzm04yrrNL0TYNaS6QVKg=s64-c-k-c0x00ffffff-no-rj", "width": 64, "height": 64}]}, "purchaseAmountText": {"simpleText": "¥200"}, "message": {"runs": [{"text": "今日初めてほんものみた、おもがんばれよ"}]}, "headerBackgroundColor": 4278237396, "headerTextColor": 4278190080, "bodyBackgroundColor": 4278248959, "bodyTextColor": 4278190080, "authorExternalChannelId": "UCdEhG3ljd_GhzzRay0i-0nQ", "authorNameTextColor": 3003121664, "contextMenuEndpoint": {"clickTrackingParams": "CAIQ7rsEIhMIg6vejuiPigMVXmr1BR0z6x8E", "commandMetadata": {"webCommandMetadata": {"ignoreNavigation": true}}, "liveChatItemContextMenuEndpoint": {"params": "Q2g0S0hBb2FRMDlsUkdkSldHOXFORzlFUm1NM1EzZG5VV1JYZUUwd1NWRWFLU29uQ2hoVlF6WnZURGhuWlV0TlpYbG5iUzAwT1dsTWNuRmxabmNTQzJkWWNrUkthakZyWjB0SklBRW9CRElhQ2hoVlEyUkZhRWN6Ykdwa1gwZG9lbnBTWVhrd2FTMHdibEU0QWtnQVVBOCUzRA=="}}, "timestampColor": 2147483648, "contextMenuAccessibility": {"accessibilityData": {"label": "Chat actions"}}, "trackingParams": "CAIQ7rsEIhMIg6vejuiPigMVXmr1BR0z6x8E", "authorBadges": [{"liveChatAuthorBadgeRenderer": {"icon": {"iconType": "MODERATOR"}, "tooltip": "Moderator", "accessibility": {"accessibilityData": {"label": "Moderator"}}}}, {"liveChatAuthorBadgeRenderer": {"customThumbnail": {"thumbnails": [{"url": "https://yt3.ggpht.com/pXR9awenP6d6R834AgGxte9GJkrUIH_JEhTQhshA55tMmthEV8smNV8GFUhqgnNAvQEaNSml5EQhIzM=s16-c-k", "width": 16, "height": 16}, {"url": "https://yt3.ggpht.com/pXR9awenP6d6R834AgGxte9GJkrUIH_JEhTQhshA55tMmthEV8smNV8GFUhqgnNAvQEaNSml5EQhIzM=s32-c-k", "width": 32, "height": 32}]}, "tooltip": "Member (2 months)", "accessibility": {"accessibilityData": {"label": "Member (2 months)"}}}}], "textInputBackgroundColor": 822083583, "creatorHeartButton": {"creatorHeartViewModel": {"creatorThumbnail": {"sources": [{"url": "https://yt3.ggpht.com/JvnR_Vv_2eMqmKEQ-_-Rqbm6JcwJmawPy8jFU60QYbLxeLNSHUWIenI-JxQDPD5j-8QTV0bC16M=s48-c-k-c0x00ffffff-no-rj"}]}, "heartedIcon": {"sources": [{"clientResource": {"imageName": "full_heart-filled"}}]}, "unheartedIcon": {"sources": [{"clientResource": {"imageName": "full_heart"}}], "processor": {"borderImageProcessor": {"imageTint": {"color": 4278190080}}}}, "heartedHoverText": "❤ by ホロライブ愛好会ch【旧:ホロライブ切り抜き hololive clips】", "heartedAccessibilityLabel": "Remove heart", "unheartedAccessibilityLabel": "Heart", "engagementStateKey": "EktsaXZlLWNoYXQtbWVzc2FnZS1lbmdhZ2VtZW50LXN0YXRlLUNod0tHa05QWlVSblNWaHZhalJ2UkVaak4wTjNaMUZrVjNoTk1FbFIgLCgB"}}, "isV2Style": true}}, "clientId": "COeDgIXoj4oDFc7CwgQdWxM0IQ"}}]}, "videoOffsetTimeMsec": "3128847", "isLive": true}
        "#;

        #[test]
        fn it_has_one_simple_chat() -> anyhow::Result<()> {
            let expected = 1;

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let actual = simple_chats.len();

            assert_eq!(expected, actual);
            Ok(())
        }

        #[test]
        fn it_equals_chat_text_message_category() -> anyhow::Result<()> {
            let expected = CategoryValue::ChatPaidMessage;

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let first = simple_chats.first().context("There is no chat")?;
            let actual = first.category.clone();

            assert_eq!(expected, actual);
            Ok(())
        }
    }

    mod live_chat_paid_sticker_renderer {
        use super::*;
        use pretty_assertions::assert_eq;

        const RAW_JSON: &str = r#"
            {"replayChatItemAction": {"actions": [{"clickTrackingParams": "CAEQl98BIhMI75ionOWPigMVIF8PAh37ORBy", "addChatItemAction": {"item": {"liveChatPaidStickerRenderer": {"id": "ChwKGkNPamo2bzNsajRvREZhYkN3Z1FkSGlBNGxn", "contextMenuEndpoint": {"clickTrackingParams": "CAIQ77sEIhMI75ionOWPigMVIF8PAh37ORBy", "commandMetadata": {"webCommandMetadata": {"ignoreNavigation": true}}, "liveChatItemContextMenuEndpoint": {"params": "Q2g0S0hBb2FRMDlxYWpadk0yeHFORzlFUm1GaVEzZG5VV1JJYVVFMGJHY2FLU29uQ2hoVlF6WnZURGhuWlV0TlpYbG5iUzAwT1dsTWNuRmxabmNTQzJkWWNrUkthakZyWjB0SklBRW9CRElhQ2hoVlEyUkZhRWN6Ykdwa1gwZG9lbnBTWVhrd2FTMHdibEU0QWtnQVVCUSUzRA=="}}, "contextMenuAccessibility": {"accessibilityData": {"label": "Chat actions"}}, "timestampUsec": "1733372891803993", "authorPhoto": {"thumbnails": [{"url": "https://yt4.ggpht.com/", "width": 32, "height": 32}, {"url": "https://yt4.ggpht.com/j", "width": 64, "height": 64}]}, "authorName": {"simpleText": "authorName"}, "authorExternalChannelId": "UCdEhG3ljd_GhzzRay0i-0nQ", "sticker": {"thumbnails": [{"url": "//lh3.googleusercontent.com/kgcJnLI6rRPD1Jm7xko7FNnl0k9qVFGzNvu8TmtTcAs4vHwigbTfa0N7N98r1TfqUPfHfRRln47UiRbeCr3Z=s40-rp", "width": 40, "height": 40}, {"url": "//lh3.googleusercontent.com/kgcJnLI6rRPD1Jm7xko7FNnl0k9qVFGzNvu8TmtTcAs4vHwigbTfa0N7N98r1TfqUPfHfRRln47UiRbeCr3Z=s80-rp", "width": 80, "height": 80}], "accessibility": {"accessibilityData": {"label": "A pile of poop with a face"}}}, "authorBadges": [{"liveChatAuthorBadgeRenderer": {"icon": {"iconType": "MODERATOR"}, "tooltip": "Moderator", "accessibility": {"accessibilityData": {"label": "Moderator"}}}}, {"liveChatAuthorBadgeRenderer": {"customThumbnail": {"thumbnails": [{"url": "https://yt3.ggpht.com/pXR9awenP6d6R834AgGxte9GJkrUIH_JEhTQhshA55tMmthEV8smNV8GFUhqgnNAvQEaNSml5EQhIzM=s16-c-k", "width": 16, "height": 16}, {"url": "https://yt3.ggpht.com/pXR9awenP6d6R834AgGxte9GJkrUIH_JEhTQhshA55tMmthEV8smNV8GFUhqgnNAvQEaNSml5EQhIzM=s32-c-k", "width": 32, "height": 32}]}, "tooltip": "Member (2 months)", "accessibility": {"accessibilityData": {"label": "Member (2 months)"}}}}], "moneyChipBackgroundColor": 4280191205, "moneyChipTextColor": 4294967295, "purchaseAmountText": {"simpleText": "¥90"}, "stickerDisplayWidth": 40, "stickerDisplayHeight": 40, "backgroundColor": 4280191205, "authorNameTextColor": 3019898879, "trackingParams": "CAIQ77sEIhMI75ionOWPigMVIF8PAh37ORBy", "isV2Style": true}}, "clientId": "COjj6o3lj4oDFabCwgQdHiA4lg"}}]}, "videoOffsetTimeMsec": "2360088", "isLive": true}
        "#;

        #[test]
        fn it_has_no_simple_chat() -> anyhow::Result<()> {
            let expected = 0;

            let json_chat = serde_json::from_str::<LiveChatEntity>(&RAW_JSON)?;
            let simple_chats: Vec<SimpleChatEntity> = json_chat.try_into()?;
            let actual = simple_chats.len();

            assert_eq!(expected, actual);

            Ok(())
        }
    }
}
