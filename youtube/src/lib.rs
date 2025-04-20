mod application;
mod domain;
mod infrastructure;
mod interface;

pub mod prelude {
    //! This module contains the prelude for the library.

    pub use crate::interface::cli::FormattedJsonCommand;
    pub use crate::interface::cli::LiveChatJsonCommand;
    pub use crate::interface::cli::WatchChatCommand;
    pub use crate::interface::presenter::StdoutPresenter;
}
