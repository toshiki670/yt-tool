use serde::{Deserialize, Serialize};

use super::{
    accessibility::{Accessibility, Label},
    icon::Icon,
    ignore_navigation::IgnoreNavigation,
    text::Text,
    web_command_metadata::WebCommandMetadata,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyButton {
    pub button_renderer: ButtonRenderer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub text: Text,
    pub icon: Icon,
    pub accessibility: Label,
    pub tracking_params: String,
    pub accessibility_data: Accessibility,
    pub command: Command,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    pub click_tracking_params: String,
    pub command_executor_command: CommandExecutorCommand,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandExecutorCommand {
    pub commands: Vec<Command2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command2 {
    pub click_tracking_params: String,
    pub log_flow_logging_event_command: Option<LogFlowLoggingEventCommand>,
    pub command_metadata: Option<WebCommandMetadata<IgnoreNavigation>>,
    pub open_super_sticker_buy_flow_command: Option<OpenSuperStickerBuyFlowCommand>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogFlowLoggingEventCommand {
    pub flow_event_type: i64,
    pub flow_event_namespace: String,
    pub flow_type: String,
    pub flow_event_metadata: FlowEventMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowEventMetadata {
    pub pdg_buy_flow_context: PdgBuyFlowContext,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdgBuyFlowContext {
    pub product_type: String,
    pub entry_point_clicked_context: EntryPointClickedContext,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryPointClickedContext {
    pub super_sticker_buy_flow_entry_point: Option<String>,
    pub super_chat_buy_flow_entry_point: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSuperStickerBuyFlowCommand {
    pub params: String,
}
