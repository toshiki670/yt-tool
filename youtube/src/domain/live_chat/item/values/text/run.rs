use super::super::thumbnails::Thumbnails;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Run {
    pub bold: Option<bool>,
    pub italics: Option<bool>,
    pub text: Option<String>,
    pub emoji: Option<Emoji>,
    pub navigation_endpoint: Option<NavigationEndpoint>,
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
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub emoji_id: String,
    pub shortcuts: Option<Vec<String>>,
    pub search_terms: Option<Vec<String>>,
    pub image: Thumbnails,
    pub is_custom_emoji: Option<bool>,
}

impl From<Emoji> for String {
    fn from(val: Emoji) -> Self {
        val.emoji_id
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata,
    pub url_endpoint: UrlEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata {
    pub web_command_metadata: WebCommandMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlEndpoint {
    pub url: String,
    pub target: String,
    pub nofollow: bool,
}
