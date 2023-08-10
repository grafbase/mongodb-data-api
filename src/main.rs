mod cli;
mod error;
mod log;
mod server;

use crate::cli::Cli;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::start()?;

    let args = Cli::parse();
    server::start(&args).await?;

    Ok(())
}
