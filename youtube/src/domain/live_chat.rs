// https://transform.tools/json-to-rust-serde
pub(crate) mod actions;
pub(crate) mod item;

use actions::{
    close_live_chat_action_panel_action::CloseLiveChatActionPanelAction, show_live_chat_action_panel_action::ShowLiveChatActionPanelAction, update_live_chat_poll_action::UpdateLiveChatPollAction
};
use item::{
    renderers::{
        live_chat_action_panel_renderer::LiveChatActionPanelRenderer,
        live_chat_banner_renderer::LiveChatBannerRenderer,
    },
    Item,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatEntity {
    pub click_tracking_params: Option<String>,
    pub is_live: Option<bool>,
    pub replay_chat_item_action: ReplayChatItemAction,
    pub video_offset_time_msec: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ReplayChatItemAction {
    pub actions: Vec<Action>,
    pub video_offset_time_msec: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Action {
    pub add_chat_item_action: Option<AddChatItemAction>,
    pub add_live_chat_ticker_item_action: Option<AddLiveChatTickerItemAction>,
    pub click_tracking_params: Option<String>,
    pub live_chat_report_moderation_state_command: Option<serde_json::Value>,
    pub remove_chat_item_action: Option<serde_json::Value>,
    pub add_banner_to_live_chat_command: Option<AddBannerToLiveChatCommand>,
    pub remove_chat_item_by_author_action: Option<serde_json::Value>,
    pub show_live_chat_action_panel_action: Option<ShowLiveChatActionPanelAction>,
    pub update_live_chat_poll_action: Option<UpdateLiveChatPollAction>,
    pub close_live_chat_action_panel_action: Option<CloseLiveChatActionPanelAction>,
    pub remove_banner_for_live_chat_command: Option<RemoveBannerForLiveChatCommand>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AddChatItemAction {
    pub client_id: Option<String>,
    pub item: Item,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AddLiveChatTickerItemAction {
    pub duration_sec: String,
    pub item: Item,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AddBannerToLiveChatCommand {
    pub banner_renderer: BannerRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BannerRenderer {
    pub live_chat_banner_renderer: Box<LiveChatBannerRenderer>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RemoveBannerForLiveChatCommand {
    pub target_action_id: String,
}
