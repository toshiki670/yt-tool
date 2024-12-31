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
        write!(f, "{}", self.into())
    }
}

impl Into<String> for Message {
    fn into(self) -> String {
        self.runs.into_iter().map(|run| run.into()).collect::<Vec<String>>().join("")
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

impl Into<String> for Run {
    fn into(self) -> String {
        self.text.unwrap_or_else(|| self.emoji.unwrap_or_default().into())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub emoji_id: String,
    pub shortcuts: Vec<String>,
    pub search_terms: Vec<String>,
    pub image: Thumbnails,
    pub is_custom_emoji: bool,
}

impl Into<String> for Emoji {
    fn into(self) -> String {
        self.shortcuts.join(", ")
    }
}
