use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub runs: Vec<Text>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub bold: Option<bool>,
    pub italics: Option<bool>,
    pub text: String,
}

impl Into <String> for Message {
    fn into(self) -> String {
        self.runs.iter().map(|run| run.text.clone()).collect()
    }
}