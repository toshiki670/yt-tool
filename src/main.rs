mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::Args::run().await?;
    Ok(())
}
