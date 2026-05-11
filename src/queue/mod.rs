mod config;
mod consume;

use futures_lite::StreamExt;
use lapin::{Connection, ConnectionProperties, options::QueueDeclareOptions};

pub use config::*;
use consume::*;
use uuid::Uuid;

pub async fn connect() -> color_eyre::Result<()> {
    let config: Config = ::config::Config::builder()
        .add_source(
            ::config::Environment::default()
                .try_parsing(true)
                .separator("__"),
        )
        .build()?
        .try_deserialize()?;
    let connection = Connection::connect(
        &config.amqp_url,
        ConnectionProperties::default().enable_auto_recover(),
    )
    .await?;

    let channel = connection.create_channel().await?;
    let _queue = channel.queue_declare(
        config.task_queue.as_str().into(),
        QueueDeclareOptions::durable(),
        Default::default(),
    );

    let mut consumer = channel
        .basic_consume(
            config.task_queue.as_str().into(),
            Uuid::new_v4().to_string().into(),
            Default::default(),
            Default::default(),
        )
        .await?;
    while let Some(delivery) = consumer.next().await {
        tracing::info!(?delivery, "received message");
        let delivery = delivery?;
        consume(&delivery.data).await?;
    }

    Ok(())
}
