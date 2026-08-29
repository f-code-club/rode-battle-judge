mod judge;
mod shared;
mod compiler;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .init();

    let queue = judge::Queue::new().await?;
    queue.listen().await?;

    Ok(())
}
