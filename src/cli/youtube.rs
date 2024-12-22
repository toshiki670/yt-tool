mod comment_file;

use crate::cli::Route;

#[derive(clap::Args, Debug)]
#[command(name = "Youtube")]
pub(super) struct Args {
    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
#[enum_delegate::implement(Route)]
enum Subcommand {
    Chat(chat::Args),
}

impl Route for Args {
    fn route(&self) -> anyhow::Result<()> {
        self.command.route()?;
        Ok(())
    }
}
