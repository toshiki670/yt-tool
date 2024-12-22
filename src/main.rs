mod cli;

fn main() -> anyhow::Result<()> {
    cli::Args::run()?;
    Ok(())
}
