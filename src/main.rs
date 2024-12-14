use clap::Parser;
use cli::{Args, Route};

mod cli;
mod youtube;

fn main() {
    Args::parse().route();
}
