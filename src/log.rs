use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn start() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
