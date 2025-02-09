use crate::domain::{
    live_chat::renderers::live_chat_action_panel_renderer::LiveChatActionPanelRenderer,
    simple_chat::SimpleChatEntity,
};

impl From<LiveChatActionPanelRenderer> for SimpleChatEntity {
    fn from(val: LiveChatActionPanelRenderer) -> Self {
        val.contents.poll_renderer.into()
    }
}
