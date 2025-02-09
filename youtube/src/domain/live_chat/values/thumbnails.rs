use super::accessibility::Accessibility;
use serde::{Deserialize, Serialize};
use url::Url;

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
    pub url: Url,
    pub width: Option<i64>,
}
