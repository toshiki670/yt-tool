mod cli;
mod utils;

fn main() -> anyhow::Result<()> {
    cli::Args::run()?;
    Ok(())
}
