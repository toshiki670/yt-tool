mod chat;

use crate::cli::Route;

#[derive(clap::Args, Debug)]
#[command(name = "Youtube")]
pub(super) struct Args {
    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    Chat(chat::Args),
}

impl Route for Args {
    async fn route(&self) -> anyhow::Result<()> {
        match &self.command {
            Subcommand::Chat(chat) => chat.route().await?,
        }
        Ok(())
    }
}
