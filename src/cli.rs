mod youtube;

use clap::{CommandFactory, Parser};
use clap_complete::{Generator, Shell, generate};
use log::{Level, info};
use std::{env, io::stdout};
use tokio::time::Instant;

trait Route {
    async fn route(&self) -> anyhow::Result<()>;
}

#[derive(Parser, Debug)]
#[command(author, version, about = "yt-dlp tool", long_about = None)]
pub(super) struct Args {
    #[arg(short, long, global = true, help = "Show logs")]
    verbose: bool,

    #[arg(long, value_name = "SHELL_NAME", help = "Generate shell completions")]
    generate_completions: Option<Shell>,

    #[command(subcommand)]
    command: Option<Subcommand>,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    Youtube(youtube::Args),
}

impl Args {
    pub(super) async fn run() -> anyhow::Result<()> {
        Args::parse().route().await?;
        Ok(())
    }
}

impl Route for Args {
    async fn route(&self) -> anyhow::Result<()> {
        if let Some(shell) = &self.generate_completions {
            generate_completions(*shell);
        } else {
            initialize_logger(self.verbose);
            let start = Instant::now();

            if let Some(command) = &self.command {
                match command {
                    Subcommand::Youtube(youtube) => youtube.route().await?,
                }
            }

            let duration = start.elapsed();
            info!("Total time: {:.3?}", duration);
        }
        Ok(())
    }
}

fn generate_completions<G: Generator>(r#gen: G) {
    let mut cmd = Args::command();
    let bin_name = cmd.get_name().to_string();
    generate(r#gen, &mut cmd, bin_name, &mut stdout());
}

fn initialize_logger(verbose: bool) {
    if verbose {
        unsafe {
            env::set_var("RUST_LOG", Level::Trace.to_string());
        }
    }
    env_logger::init();
}
