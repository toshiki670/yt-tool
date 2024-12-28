use serde::{Deserialize, Serialize};

use super::{accessibility::Accessibility, icon::Icon, thumbnails::Thumbnails};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorBadge {
    pub live_chat_author_badge_renderer: LiveChatAuthorBadgeRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatAuthorBadgeRenderer {
    pub accessibility: Accessibility,
    pub custom_thumbnail: Option<Thumbnails>,
    pub icon: Option<Icon>,
    pub tooltip: String,
}
