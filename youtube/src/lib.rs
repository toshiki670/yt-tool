mod application;
mod domain;
mod infrastructure;
mod interface;

pub mod prelude {
    //! This module contains the prelude for the library.

    pub use crate::interface::formatted_json_service::*;
    pub use crate::interface::live_chat_json_service::*;
}
