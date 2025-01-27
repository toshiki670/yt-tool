use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::thumbnails::Thumbnails;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub runs: Vec<Run>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = self.clone().into();
        write!(f, "{}", message)
    }
}

impl From<Message> for String {
    fn from(val: Message) -> Self {
        val.runs
            .into_iter()
            .map(|run| run.into())
            .collect::<Vec<String>>()
            .join("")
    }
}

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
