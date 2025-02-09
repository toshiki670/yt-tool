use super::super::thumbnails::Thumbnails;
use crate::domain::live_chat::values::web_command_metadata::WebCommandMetadata;
use log::warn;
use serde::{Deserialize, Serialize};
use url::Url;

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
            endpoint.url_endpoint.unwrap_redirect().url.to_string()
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub click_tracking_params: Option<String>,
    pub command_metadata: WebCommandMetadata<NavigationMetadata>,
    pub url_endpoint: UrlEndpoint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct NavigationMetadata {
    pub url: Url,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UrlEndpoint {
    pub url: Url,
    pub target: String,
    pub nofollow: bool,
}

impl UrlEndpoint {
    fn is_youtube_redirect(&self) -> bool {
        self.url.domain() == Some("www.youtube.com") && self.url.path() == "/redirect"
    }

    pub(super) fn unwrap_redirect(mut self) -> Self {
        if self.is_youtube_redirect() {
            let mut params = self.url.query_pairs();
            if let Some(redirect_url) = params.find(|(key, _)| key == "q").map(|(_, value)| value) {
                if let Ok(url) = Url::parse(&redirect_url) {
                    self.url = url;
                    return self;
                } else {
                    warn!(
                        "Failed to parse redirect URL {:?} of {:?}",
                        &redirect_url, &self.url
                    );
                }
            } else {
                warn!("No redirect URL found in {:?}", &self.url);
            }
        }
        self
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSelectorThumbnailRow {
    pub thumbnails: Vec<Thumbnails>,
}
