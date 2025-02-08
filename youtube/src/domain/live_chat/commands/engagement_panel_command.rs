use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EngagementPanelCommand {
    pub click_tracking_params: String,
    pub show_engagement_panel_endpoint: ShowEngagementPanelEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShowEngagementPanelEndpoint {
    pub engagement_panel_presentation_configs: EngagementPanelPresentationConfigs,
    pub global_configuration: GlobalConfiguration,
    pub identifier: Identifier,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Identifier {
    pub surface: String,
    pub tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GlobalConfiguration {
    pub params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EngagementPanelPresentationConfigs {
    pub engagement_panel_popup_presentation_config: EngagementPanelPopupPresentationConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EngagementPanelPopupPresentationConfig {
    pub popup_type: String,
}
