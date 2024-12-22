mod cli;
mod youtube;

fn main() -> anyhow::Result<()> {
    cli::Args::run();
    Ok(())
}
