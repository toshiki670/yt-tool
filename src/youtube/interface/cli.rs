mod comment;

use crate::cli::Route;
use clap::{Args, Subcommand};
use comment::CommentArgs;
use enum_dispatch::enum_dispatch;

#[derive(Args, Debug)]
#[command(name = "Youtube")]
pub struct YoutubeArgs {
    #[command(subcommand)]
    command: YoutubeSubcommand,
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(Route)]
pub enum YoutubeSubcommand {
    Comment(CommentArgs),
}

impl Route for YoutubeArgs {
    fn route(&self) {
        self.command.route();
    }
}
