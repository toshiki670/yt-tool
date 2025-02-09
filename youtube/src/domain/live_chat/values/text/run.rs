use super::super::thumbnails::Thumbnails;
use crate::domain::live_chat::values::web_command_metadata::WebCommandMetadata;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Run {
    pub bold: Option<bool>,
    pub emoji: Option<Emoji>,
    pub italics: Option<bool>,
    pub navigation_endpoint: Option<NavigationEndpoint>,
    pub text: Option<String>,
}

impl From<Run> for String {
    fn from(val: Run) -> Self {
        if let Some(endpoint) = val.navigation_endpoint {
            endpoint.url_endpoint.url
        } else if let Some(text) = val.text {
            text
        } else if let Some(emoji) = val.emoji {
            emoji.into()
        } else {
            unreachable!()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Emoji {
    pub emoji_id: String,
    pub image: Thumbnails,
    pub is_custom_emoji: Option<bool>,
    pub search_terms: Option<Vec<String>>,
    pub shortcuts: Option<Vec<String>>,
    pub supports_skin_tone: Option<bool>,
    pub variant_ids: Option<Vec<String>>,
    pub multi_selector_thumbnail_row: Option<Vec<MultiSelectorThumbnailRow>>,
}

impl From<Emoji> for String {
    fn from(val: Emoji) -> Self {
        if !val.is_custom_emoji.unwrap_or(false) {
            return val.emoji_id;
        }
        if let Some(s) = val.shortcuts.as_ref().and_then(|s| s.first()) {
            s.clone()
        } else if let Some(s) = val.search_terms.as_ref().and_then(|s| s.first()) {
            s.clone()
        } else {
            val.emoji_id
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub click_tracking_params: Option<String>,
    pub command_metadata: WebCommandMetadata<NavigationMetadata>,
    pub url_endpoint: UrlEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct NavigationMetadata {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UrlEndpoint {
    pub url: String,
    pub target: String,
    pub nofollow: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSelectorThumbnailRow {
    pub thumbnails: Vec<Thumbnails>,
}
