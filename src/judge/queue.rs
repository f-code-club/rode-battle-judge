use std::collections::HashMap;

use futures_lite::StreamExt;
use lapin::{
    Channel, Connection, ConnectionProperties,
    options::{BasicNackOptions, QueueDeclareOptions},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    judge::{config::Config, service},
    shared::{Storage, database},
};

const MAX_RETRIES: u64 = 3;

pub struct Queue {
    pub task_queue: String,
    pub channel: Channel,
    pub pool: PgPool,
    pub storage: Storage,
    pub retry_cnt: HashMap<Uuid, u64>,
}

impl Queue {
    pub async fn new() -> color_eyre::Result<Self> {
        let cfg = Config::new()?;

        let (channel, pool, storage) = tokio::try_join!(
            async {
                let connection = Connection::connect(
                    &cfg.amqp_url,
                    ConnectionProperties::default().enable_auto_recover(),
                )
                .await?;

                let channel = connection.create_channel().await?;
                let _ = channel
                    .queue_declare(
                        cfg.task_queue.as_str().into(),
                        QueueDeclareOptions::durable(),
                        Default::default(),
                    )
                    .await;

                Ok(channel)
            },
            database::connect(),
            Storage::new()
        )?;

        tracing::info!("queue connected");

        Ok(Self {
            task_queue: cfg.task_queue,
            channel,
            pool,
            storage,
            retry_cnt: HashMap::new(),
        })
    }

    pub async fn listen(&mut self) -> color_eyre::Result<()> {
        tracing::info!("waiting for task");

        let mut consumer = self
            .channel
            .basic_consume(
                self.task_queue.as_str().into(),
                Uuid::new_v4().to_string().into(),
                Default::default(),
                Default::default(),
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            tracing::info!(?delivery, "received message");
            let delivery = delivery?;

            let id = match Uuid::from_slice(&delivery.data) {
                Ok(id) => id,
                Err(error) => {
                    tracing::error!(?error, "invalid submission id");

                    delivery
                        .nack(BasicNackOptions {
                            requeue: false,
                            ..Default::default()
                        })
                        .await?;

                    continue;
                }
            };
            let cnt = self.retry_cnt.entry(id).or_insert(0);
            if *cnt == MAX_RETRIES {
                delivery
                    .nack(BasicNackOptions {
                        requeue: false,
                        ..Default::default()
                    })
                    .await?;
                continue;
            }
            *cnt += 1;

            match service::run(&self.storage, &self.pool, id).await {
                Err(error) => {
                    tracing::error!(?error, "failed to process message");

                    delivery
                        .nack(BasicNackOptions {
                            requeue: true,
                            ..Default::default()
                        })
                        .await?
                }
                _ => delivery.ack(Default::default()).await?,
            };
        }

        Ok(())
    }
}
