use serde::{Deserialize, Serialize};

use crate::domain::live_chat::renderers::banner_renderer::BannerRenderer;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AddBannerToLiveChatCommand {
    pub banner_renderer: BannerRenderer,
}
