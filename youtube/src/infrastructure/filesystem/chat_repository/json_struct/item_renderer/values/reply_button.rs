use super::engagement_panel_command::EngagementPanelCommand;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyButton {
    pub pdg_reply_button_view_model: PdgReplyButtonViewModel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdgReplyButtonViewModel {
    pub reply_button: ReplyButton2,
    pub reply_count_entity_key: String,
    pub reply_count_placeholder: ReplyCountPlaceholder,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyButton2 {
    pub button_view_model: ButtonViewModel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonViewModel {
    pub icon_name: String,
    pub on_tap: OnTap,
    pub accessibility_text: String,
    pub style: String,
    pub tracking_params: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub button_size: String,
    pub custom_background_color: i64,
    pub custom_font_color: i64,
    pub logging_directives: LoggingDirectives,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnTap {
    pub innertube_command: EngagementPanelCommand,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingDirectives {
    pub tracking_params: String,
    pub visibility: Visibility,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visibility {
    pub types: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyCountPlaceholder {
    pub content: String,
    pub style_runs: Vec<StyleRun>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleRun {
    pub start_index: i64,
    pub length: i64,
}
