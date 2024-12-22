use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub is_live: bool,
    pub replay_chat_item_action: ReplayChatItemAction,
    pub video_offset_time_msec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayChatItemAction {
    pub actions: Vec<Action>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub add_chat_item_action: AddChatItemAction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddChatItemAction {
    pub client_id: String,
    pub item: Item,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Item {
    #[default]
    None,
    LiveChatSponsorshipsGiftRedemptionAnnouncementRenderer(LiveChatRenderer),
    LiveChatTextMessageRenderer(LiveChatRenderer),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatRenderer {
    pub author_badges: Option<Vec<AuthorBadge>>,
    pub author_external_channel_id: String,
    pub author_name: AuthorName,
    pub author_photo: Thumbnails,
    pub context_menu_accessibility: Accessibility,
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub id: String,
    pub message: Message,
    pub timestamp_usec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility {
    pub accessibility_data: AccessibilityData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorBadge {
    pub live_chat_author_badge_renderer: LiveChatAuthorBadgeRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatAuthorBadgeRenderer {
    pub accessibility: Accessibility,
    pub custom_thumbnail: Option<Thumbnails>,
    pub icon: Option<Icon>,
    pub tooltip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorName {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextMenuEndpoint {
    pub command_metadata: CommandMetadata,
    pub live_chat_item_context_menu_endpoint: LiveChatItemContextMenuEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata {
    pub web_command_metadata: WebCommandMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata {
    pub ignore_navigation: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatItemContextMenuEndpoint {
    pub params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub icon_type: IconType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IconType {
    #[default]
    VERIFIED,
    MODERATOR,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub runs: Vec<Run>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub bold: Option<bool>,
    pub italics: Option<bool>,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub height: i64,
    pub url: String,
    pub width: i64,
}
