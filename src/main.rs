use clap::Parser;
use cli::{CliArgs, Route};

mod cli;
mod youtube;

fn main() {
    CliArgs::parse().route();
}
