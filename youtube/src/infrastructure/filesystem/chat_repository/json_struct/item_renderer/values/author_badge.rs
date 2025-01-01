use serde::{Deserialize, Serialize};

use super::{accessibility::Accessibility, icon::Icon, thumbnails::Thumbnails};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorBadges(Vec<AuthorBadge>);

impl core::ops::Deref for AuthorBadges {
    type Target = Vec<AuthorBadge>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.0
    }
}

impl AuthorBadges {
    pub fn has_moderator(&self) -> bool {
        self.iter().any(|badge| badge.is_moderator())
    }
}

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

impl AuthorBadge {
    pub fn is_moderator(&self) -> bool {
        if let Some(icon) = &self.live_chat_author_badge_renderer.icon {
            icon.is_moderator()
        } else {
            false
        }
    }
}
