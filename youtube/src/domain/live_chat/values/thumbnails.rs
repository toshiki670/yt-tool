use super::accessibility::Accessibility;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thumbnails {
    pub accessibility: Option<Accessibility>,
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thumbnail {
    pub height: Option<i64>,
    pub url: String,
    pub width: Option<i64>,
}
