mod config;
mod consume;

use lapin::{
    Connection, ConnectionProperties, message::DeliveryResult, options::QueueDeclareOptions,
};

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

    let consumer = channel
        .basic_consume(
            config.task_queue.as_str().into(),
            Uuid::new_v4().to_string().into(),
            Default::default(),
            Default::default(),
        )
        .await?;
    consumer.set_delegate(move |delivery: DeliveryResult| async move {
        let delivery = match delivery {
            Ok(Some(delivery)) => delivery,
            Ok(None) => return,
            Err(error) => {
                tracing::error!(?error, "failed to consume message");
                return;
            }
        };

        if let Err(error) = consume(&delivery.data).await {
            tracing::error!(?error, "failed to process data");
        };

        if let Err(error) = delivery.ack(Default::default()).await {
            tracing::error!(?error, "failed to ack message");
        }
    });

    Ok(())
}
