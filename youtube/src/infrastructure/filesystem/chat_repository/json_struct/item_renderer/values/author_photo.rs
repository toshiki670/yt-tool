use super::{accessibility::Accessibility, thumbnails::Thumbnail};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorPhoto {
    pub thumbnails: Vec<Thumbnail>,
    pub accessibility: Accessibility,
}
