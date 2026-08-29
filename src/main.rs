mod compiler;
mod judge;
mod shared;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .init();

    let queue = judge::Queue::new().await?;

    tokio::try_join!(queue.listen(), compiler::run())?;

    Ok(())
}
