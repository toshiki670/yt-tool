pub(super) mod run;
use run::Run;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Text {
    #[default]
    None,
    #[allow(clippy::enum_variant_names)]
    SimpleText(String),
    Runs(Vec<Run>),
}

impl From<Text> for String {
    fn from(val: Text) -> Self {
        match val {
            Text::SimpleText(text) => text,
            Text::Runs(runs) => runs
                .into_iter()
                .map(|run| run.into())
                .collect::<Vec<String>>()
                .join(""),
            Text::None => String::new(),
        }
    }
}
