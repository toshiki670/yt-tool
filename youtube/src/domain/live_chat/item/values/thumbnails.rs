use serde::{Deserialize, Serialize};

use super::accessibility::Accessibility;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
    pub accessibility: Option<Accessibility>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub height: Option<i64>,
    pub url: String,
    pub width: Option<i64>,
}
