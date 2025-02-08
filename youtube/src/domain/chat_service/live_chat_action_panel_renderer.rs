use crate::domain::{
    live_chat::item::renderers::live_chat_action_panel_renderer::LiveChatActionPanelRenderer,
    simple_chat::SimpleChatEntity,
};

impl From<Box<LiveChatActionPanelRenderer>> for SimpleChatEntity {
    fn from(val: Box<LiveChatActionPanelRenderer>) -> Self {
        val.contents.poll_renderer.into()
    }
}
