use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreatorHeartButton {
    pub creator_heart_view_model: CreatorHeartViewModel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreatorHeartViewModel {
    pub creator_thumbnail: Sources<SourceUrl>,
    pub engagement_state_key: String,
    pub gradient: Option<serde_json::Value>,
    pub hearted_accessibility_label: String,
    pub hearted_hover_text: String,
    pub hearted_icon: Sources<ClientResource>,
    pub unhearted_accessibility_label: String,
    pub unhearted_icon: UnheartedIcon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Sources<T> {
    pub sources: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SourceUrl {
    pub url: Url,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ClientResource {
    pub client_resource: ImageName,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImageName {
    pub image_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnheartedIcon {
    pub sources: Vec<ClientResource>,
    pub processor: BorderImageProcessor,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BorderImageProcessor {
    pub border_image_processor: ImageTint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImageTint {
    pub image_tint: Color,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Color {
    pub color: i64,
}
