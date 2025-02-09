use serde::{Deserialize, Serialize};

use super::live_chat_banner_renderer::LiveChatBannerRenderer;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BannerRenderer {
    pub live_chat_banner_renderer: LiveChatBannerRenderer,
}
