mod application;
mod domain;
mod infrastructure;
mod interface;

pub mod prelude {
    pub use crate::interface::chat_file_service::*;
    pub use crate::interface::chat_in_memory_service::*;
}
