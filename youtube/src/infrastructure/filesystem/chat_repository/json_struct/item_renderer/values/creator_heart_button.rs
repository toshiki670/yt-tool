use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorHeartButton {
    pub creator_heart_view_model: CreatorHeartViewModel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorHeartViewModel {
    pub creator_thumbnail: Sources<Url>,
    pub hearted_icon: Sources<ClientResource>,
    pub unhearted_icon: UnheartedIcon,
    pub hearted_hover_text: String,
    pub hearted_accessibility_label: String,
    pub unhearted_accessibility_label: String,
    pub engagement_state_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sources<T> {
    pub sources: Vec<T>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientResource {
    pub client_resource: ImageName,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageName {
    pub image_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnheartedIcon {
    pub sources: Vec<ClientResource>,
    pub processor: BorderImageProcessor,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorderImageProcessor {
    pub border_image_processor: ImageTint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageTint {
    pub image_tint: Color,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub color: i64,
}
