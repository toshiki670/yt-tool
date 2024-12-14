mod comment;

use crate::cli::Route;
use comment::CommentArgs;

#[derive(clap::Args, Debug)]
#[command(name = "Youtube")]
pub struct Args {
    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
#[enum_delegate::implement(Route)]
pub enum Subcommand {
    Comment(CommentArgs),
}

impl Route for Args {
    fn route(&self) {
        self.command.route();
    }
}
