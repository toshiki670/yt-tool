use crate::youtube::interface::cli::YoutubeArgs;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use env_logger;
use log::Level;
use std::{env, io::stdout};

#[enum_delegate::register]
pub trait Route {
    fn route(&self);
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Algorithm tool", long_about = None)]
pub struct CliArgs {
    #[arg(short, long, global = true, help = "Show logs")]
    verbose: bool,

    #[arg(long, value_name = "SHELL_NAME", help = "Generate shell completions")]
    generate_completions: Option<Shell>,

    #[command(subcommand)]
    command: Option<CliSubcommand>,
}

#[derive(Subcommand, Debug)]
#[enum_delegate::implement(Route)]
enum CliSubcommand {
    Youtube(YoutubeArgs),
}

impl Route for CliArgs {
    fn route(&self) {
        if let Some(shell) = &self.generate_completions {
            generate_completions(shell.clone());
        } else {
            initialize_logger(self.verbose);

            if let Some(command) = &self.command {
                command.route();
            }
        }
    }
}

fn generate_completions<G: Generator>(gen: G) {
    let mut cmd = CliArgs::command();
    let bin_name = cmd.get_name().to_string();
    generate(gen, &mut cmd, bin_name, &mut stdout());
}

fn initialize_logger(verbose: bool) {
    if verbose {
        env::set_var("RUST_LOG", Level::Trace.to_string());
    }
    env_logger::init();
}
