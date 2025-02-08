use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Icon {
    pub icon_type: IconType,
}

impl Icon {
    pub fn is_moderator(&self) -> bool {
        self.icon_type == IconType::Moderator
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IconType {
    Keep,
    Moderator,
    MoreVert,
    Owner,
    Poll,
    Verified,
    YoutubeRound,
}
