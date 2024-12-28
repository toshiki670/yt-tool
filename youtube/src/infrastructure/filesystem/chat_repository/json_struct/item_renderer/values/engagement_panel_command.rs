use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngagementPanelCommand {
    pub click_tracking_params: String,
    pub show_engagement_panel_endpoint: ShowEngagementPanelEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowEngagementPanelEndpoint2 {
    pub identifier: Identifier2,
    pub global_configuration: GlobalConfiguration2,
    pub engagement_panel_presentation_configs: EngagementPanelPresentationConfigs2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier2 {
    pub surface: String,
    pub tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalConfiguration2 {
    pub params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngagementPanelPresentationConfigs2 {
    pub engagement_panel_popup_presentation_config: EngagementPanelPopupPresentationConfig2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngagementPanelPopupPresentationConfig2 {
    pub popup_type: String,
}
