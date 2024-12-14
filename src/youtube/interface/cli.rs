mod comment_file;

use crate::cli::Route;

#[derive(clap::Args, Debug)]
#[command(name = "Youtube")]
pub struct Args {
    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
#[enum_delegate::implement(Route)]
pub enum Subcommand {
    CommentFile(comment_file::Args),
}

impl Route for Args {
    fn route(&self) {
        self.command.route();
    }
}
