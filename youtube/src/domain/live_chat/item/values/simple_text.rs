use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleText {
    pub simple_text: String,
}

impl From<SimpleText> for String {
    fn from(val: SimpleText) -> Self {
        val.simple_text
    }
}
