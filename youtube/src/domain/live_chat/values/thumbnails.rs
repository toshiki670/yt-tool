use serde::{Deserialize, Serialize};

use super::accessibility::Accessibility;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thumbnails {
    pub accessibility: Option<Accessibility>,
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thumbnail {
    pub height: Option<i64>,
    pub url: String,
    pub width: Option<i64>,
}
