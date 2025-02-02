use serde::{Deserialize, Serialize};
use super::super::thumbnails::Thumbnails;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub bold: Option<bool>,
    pub italics: Option<bool>,
    pub text: Option<String>,
    pub emoji: Option<Emoji>,
}

impl From<Run> for String {
    fn from(val: Run) -> Self {
        val.text
            .unwrap_or_else(|| val.emoji.unwrap_or_default().into())
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
