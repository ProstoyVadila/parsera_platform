use std::str;

use anyhow::Result;
use deadpool_lapin::lapin::options::{
    BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, ExchangeBindOptions,
    ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use deadpool_lapin::lapin::protocol::channel;
use deadpool_lapin::lapin::{types::FieldTable, BasicProperties, Channel, ExchangeKind};
use deadpool_lapin::{Config, Manager, Pool, Runtime};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use tokio_stream::StreamExt;

use crate::config::{BrokerConfig, DbAddr};

pub struct Rabbit {
    pub cfg: BrokerConfig,
    pub pool: Pool,
}

impl Rabbit {
    pub async fn new(cfg: BrokerConfig) -> Result<Self> {
        let mut pool_cfg: Config = Config::default();
        pool_cfg.url = Some(cfg.get_addr());
        let pool = pool_cfg.create_pool(Some(Runtime::Tokio1))?;
        let rabbit = Rabbit { pool, cfg };
        rabbit.declare_all().await;
        Ok(rabbit)
    }

    pub async fn get_channel(&self) -> Channel {
        let conn = self.pool.get().await.expect("conn err");
        let channel = conn.create_channel().await.expect("channel err");
        channel
    }

    pub async fn declare_all(&self) {
        let channel = self.get_channel().await;
        self.declare_exchange(channel.clone()).await;

        let mut queues = self.cfg.queues_to_produce();
        queues.push(self.cfg.queue_to_consume());

        for queue in queues {
            tracing::debug!("declaring queue {}", queue);
            self.declare_queue(channel.clone(), queue.clone()).await;
            self.bind_queue(channel.clone(), queue).await;
        }
    }

    pub async fn declare_exchange(&self, channel: Channel) {
        channel
            .exchange_declare(
                &self.cfg.exchange,
                ExchangeKind::Direct,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("cannot declare exchange");
    }

    pub async fn declare_queue(&self, channel: Channel, queue: String) {
        channel
            .queue_declare(
                &queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("cannot declare queue");
    }

    pub async fn bind_queue(&self, channel: Channel, queue: String) {
        channel
            .queue_bind(
                &queue,
                &self.cfg.exchange,
                &queue,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("cannot bind queue");
    }

    fn choose_queue(&self) -> String {
        if self.cfg.queues_to_produce().len() > 1 {
            let queues = self.cfg.queues_to_produce();
            queues
                .choose(&mut rand::thread_rng())
                .expect("cannot get random queue")
                .clone()
        } else {
            let queue = self
                .cfg
                .queues_to_produce()
                .pop()
                .expect("cannot get the first queue");
            queue
        }
    }

    pub async fn publish(&self, payload: &[u8]) {
        let channel = self.get_channel().await;
        let queue = self.choose_queue();

        let confirm = channel
            .basic_publish(
                &self.cfg.exchange,
                queue.as_str(),
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
            .expect("cannot send message to the queue");
    }

    pub async fn consume(&self) -> Result<()> {
        let queue = &self.cfg.queue_to_consume();
        tracing::info!("consuming from queue {}", queue);
        let channel = self.get_channel().await;
        let mut consumer = channel
            .basic_consume(
                queue,
                "consumer tag",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    let event = str::from_utf8(&delivery.data)
                        .expect("error converting message from consumer");
                    // TODO: impl handling event
                    tracing::info!("Got event from consumer: {}", event);
                    channel
                        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                        .await?;
                }
                Err(err) => {
                    tracing::error!("Error caught in consumer: {}", err);
                    break;
                }
            }
        }
        Ok(())
    }
}
