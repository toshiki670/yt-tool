use serde::{Deserialize, Serialize};

use super::{
    live_chat_sponsorships_header_renderer::LiveChatSponsorshipsHeaderRenderer,
    values::timestamp_usec::TimestampUsec,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LiveChatSponsorshipsGiftPurchaseAnnouncementRenderer {
    pub author_external_channel_id: String,
    pub header: Header,
    pub id: String,
    pub timestamp_usec: TimestampUsec,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Header {
    pub live_chat_sponsorships_header_renderer: LiveChatSponsorshipsHeaderRenderer,
}
