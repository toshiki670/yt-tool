use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub icon_type: IconType,
}

impl Icon {
    pub fn is_moderator(&self) -> bool {
        self.icon_type == IconType::Moderator
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IconType {
    #[default]
    Verified,
    Moderator,
    YoutubeRound,
}
