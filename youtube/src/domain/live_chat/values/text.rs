pub(super) mod run;
use std::fmt::Display;

use run::Run;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum Text {
    #[allow(clippy::enum_variant_names)]
    SimpleText(String),
    Runs(Vec<Run>),
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: String = self.clone().into();
        write!(f, "{}", text)
    }
}

impl From<Text> for String {
    fn from(val: Text) -> Self {
        let text = match val {
            Text::SimpleText(text) => text,
            Text::Runs(runs) => runs
                .into_iter()
                .map(|run| run.into())
                .collect::<Vec<String>>()
                .join(""),
        };
        text.replace("\n", "<br>")
    }
}
