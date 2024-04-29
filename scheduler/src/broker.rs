use std::str;

use anyhow::{Result, anyhow};
use deadpool_lapin::lapin::options::{
    BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, ExchangeBindOptions,
    ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use deadpool_lapin::lapin::protocol::channel;
use deadpool_lapin::lapin::publisher_confirm::PublisherConfirm;
use deadpool_lapin::lapin::ConnectionProperties;
use deadpool_lapin::lapin::{types::FieldTable, BasicProperties, Channel, ExchangeKind};
use deadpool_lapin::{Config, Manager, Pool, PoolConfig, Runtime};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use tokio_stream::StreamExt;

use common::{retry, increasing_retry};

use crate::config::{BrokerConfig, DbAddr};
use crate::{scheduler, SharedSheduler};
use crate::utils::ParseraService;

impl ParseraService {
    fn routing_key<'a>(&'a self, cfg: &'a BrokerConfig) -> &str {
        match self {
            ParseraService::Scraper => &cfg.scraper_queue,
            ParseraService::HeavyArtillery => &cfg.heavy_artillery_queue,
            ParseraService::Extractor => &cfg.extractor_queue,
            ParseraService::Notification => &cfg.notification_queue,
            ParseraService::DatabaseManager => &cfg.db_manager_queue,
            ParseraService::StatusManager => &cfg.status_manager_queue,
        }
    }
}

pub struct Rabbit {
    pub cfg: BrokerConfig,
    pub pool: Pool,
}

impl Rabbit {
    pub async fn new(cfg: BrokerConfig) -> Result<Self> {
        let deadpool_config = PoolConfig::new(cfg.pool_max_size.into());
        
        let mut pool_cfg: Config = Config::default();
        pool_cfg.pool = Some(deadpool_config);
        pool_cfg.url = Some(cfg.get_addr());
        let pool = pool_cfg.create_pool(Some(Runtime::Tokio1))?;
    
        let rabbit = Rabbit { pool, cfg };
        rabbit.declare_all().await;

        Ok(rabbit)
    }

    pub async fn get_channel(&self) -> Result<Channel> {
        let conn = retry!("get_rabbit_conn", self.pool.get().await, 10, 0.2)?;
        let channel = retry!("get_rabbit_channel", conn.create_channel().await, 10, 0.2)?;
        Ok(channel)
    }

    pub async fn declare_all(&self) {
        let channel = self.get_channel().await.expect("cannot get a channel to declare");
        // Creating and binding produce exchange and queues
        self.declare_exchange(channel.clone(), &self.cfg.produce_exchange, ExchangeKind::Topic).await; 
        for queue in self.cfg.queues_to_produce() {
            tracing::debug!("declaring queue {}", queue);
            self.declare_queue(channel.clone(), &queue).await;
            self.bind_queue(channel.clone(), &self.cfg.produce_exchange, &queue).await;
        }
        // Creating and binding consume cxchange and queue
        self.declare_exchange(channel.clone(), &self.cfg.consume_exchange, ExchangeKind::Fanout).await;
        self.declare_queue(channel.clone(), &self.cfg.queue_to_consume()).await;
        self.bind_queue(channel, &self.cfg.consume_exchange, &self.cfg.queue_to_consume()).await
    }

    pub async fn declare_exchange(&self, channel: Channel, exchange: &str, kind: ExchangeKind) {
        channel
            .exchange_declare(
                exchange,
                kind,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("cannot declare an exchange");
    }

    pub async fn declare_queue(&self, channel: Channel, queue: &str) {
        channel
            .queue_declare(
                queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("cannot declare a queue");
    }

    pub async fn bind_queue(&self, channel: Channel, exchange: &str, queue: &str) {
        channel
            .queue_bind(
                queue,
                exchange,
                queue,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("cannot bind a queue");
    }

    fn choose_queue(&self) -> &str {
        if self.cfg.queues_to_produce().len() > 1 {
            let queues = self.cfg.queues_to_produce();
            queues
                .choose(&mut rand::thread_rng())
                .expect("cannot get a random queue")
        } else {
            let queue = self
                .cfg
                .queues_to_produce()
                .pop()
                .expect("cannot get the first queue");
            queue
        }
    }

    pub async fn publish(&self, payload: &[u8], to: ParseraService) -> Result<PublisherConfirm> {
        let channel = self.get_channel().await?;

        increasing_retry!(
            "publish",
            channel
                .basic_publish(
                    &self.cfg.produce_exchange,
                    to.routing_key(&self.cfg),
                    BasicPublishOptions::default(),
                    payload,
                    BasicProperties::default(),
                )
                .await,
            8,
            0.1,
            0.4
            ).map_err(|err| anyhow!(err))
    }

    pub async fn consume(&self, sched: SharedSheduler) -> Result<()> {
        let queue = &self.cfg.queue_to_consume();
        tracing::info!("consuming from queue {}", queue);
        let channel = self.get_channel().await.expect("cannot get a rabbit channel for consumer");

        let mut consumer = increasing_retry!(
            "basic_consume",
            channel
                .basic_consume(
                    queue,
                    "scheduler",
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await,
            30,
            1.0,
            2.0
        ).expect("cannot get a consumer");

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    scheduler::handle_event(self, sched.clone(), &delivery.data).await;
                    // TODO: figure out this error handling
                    channel
                        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                        .await?;
                }
                Err(err) => {
                    tracing::error!("error caught in consumer: {}", err);
                }
            }
        }
        Ok(())
    }
}
