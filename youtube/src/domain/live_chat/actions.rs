pub(crate) mod add_chat_item_action;
pub(crate) mod add_live_chat_ticker_item_action;
pub(crate) mod close_live_chat_action_panel_action;
pub(crate) mod show_live_chat_action_panel_action;
pub(crate) mod update_live_chat_poll_action;

use super::commands::{
    add_banner_to_live_chat_command::AddBannerToLiveChatCommand,
    remove_banner_for_live_chat_command::RemoveBannerForLiveChatCommand,
};
use add_chat_item_action::AddChatItemAction;
use add_live_chat_ticker_item_action::AddLiveChatTickerItemAction;
use close_live_chat_action_panel_action::CloseLiveChatActionPanelAction;
use serde::{Deserialize, Serialize};
use show_live_chat_action_panel_action::ShowLiveChatActionPanelAction;
use update_live_chat_poll_action::UpdateLiveChatPollAction;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Actions {
    pub actions: Vec<Action>,
    pub video_offset_time_msec: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Action {
    pub add_banner_to_live_chat_command: Option<AddBannerToLiveChatCommand>,
    pub add_chat_item_action: Option<AddChatItemAction>,
    pub add_live_chat_ticker_item_action: Option<AddLiveChatTickerItemAction>,
    pub click_tracking_params: Option<String>,
    pub close_live_chat_action_panel_action: Option<CloseLiveChatActionPanelAction>,
    pub live_chat_report_moderation_state_command: Option<serde_json::Value>,
    pub remove_banner_for_live_chat_command: Option<RemoveBannerForLiveChatCommand>,
    pub remove_chat_item_action: Option<serde_json::Value>,
    pub remove_chat_item_by_author_action: Option<serde_json::Value>,
    pub show_live_chat_action_panel_action: Option<ShowLiveChatActionPanelAction>,
    pub update_live_chat_poll_action: Option<UpdateLiveChatPollAction>,
}
