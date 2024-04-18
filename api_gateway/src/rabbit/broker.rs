#![allow(dead_code,unused)]
use std::process::exit;

use bb8::{ManageConnection, Pool, PooledConnection};
use bb8_lapin::{lapin, lapin::ConnectionProperties, LapinConnectionManager};
use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Error, ExchangeKind};
use lapin::publisher_confirm::PublisherConfirm;

use crate::config::{Broker as BrokerConfig, DbAddr};
use crate::utils::{retry_inc_on_err, retry_on_err};


pub type RabbitConn<'a> = PooledConnection<'a, LapinConnectionManager>;
pub type RabbitPool = Pool<LapinConnectionManager>;




pub struct RabbitMQ {
    pool: RabbitPool,
    config: BrokerConfig,
}

impl RabbitMQ {
    pub async fn new(config: BrokerConfig) -> Self {
        let pool = Self::create_rabbitmq_pool(config.get_dsn()).await;
        let broker = RabbitMQ { pool , config };

        let channel = broker.get_channel().await.unwrap();
        broker.declare_exchange(channel.clone()).await;
        broker.declare_queue(channel).await;
        broker
    }

    async fn create_rabbitmq_pool(addr: String) -> RabbitPool{
        let manager = LapinConnectionManager::new(&addr, ConnectionProperties::default());
        let pool = Pool::builder()
            .max_size(16)
            .build(manager)
            .await
            .unwrap();
        pool
    }

    pub async fn try_conn(&self) -> PooledConnection<'_, LapinConnectionManager> {
        let conn = match retry_inc_on_err!("try rabbitmq conn", self.pool.get().await) {
            Ok(conn) => conn,
            Err(err) => {
                tracing::error!("cannot connect to rabbitmq: {}", err);
                exit(1);
            }
        };
        conn
    }

    pub async fn get_channel(&self) -> Result<Channel, Error> {
        let conn = self.try_conn().await;
        let channel = match retry_on_err!("try rabbitmq channel", conn.create_channel().await) {
            Ok(ch) => ch,
            Err(err) => {
                tracing::error!("cannot get a rabbitmq channel: {}", err);
                return Err(err);
            }
        };
        Ok(channel)
    }

    async fn declare_exchange(&self, channel: Channel) {
        let _ = channel.exchange_declare(
            &self.config.exchange,
            ExchangeKind::Direct,
            ExchangeDeclareOptions::default(),
            FieldTable::default()
        ).await
         .expect("cannot declare exchange");
    }

    async fn declare_queue(&self, channel: Channel) {
        // let _ = channel.queue_bind(queue, exchange, routing_key, options, arguments)
        let _ = channel.queue_declare(
            &self.config.queue, 
            QueueDeclareOptions::default(), 
            FieldTable::default()
        ).await
         .expect("cannot declare a queue");
    }

    pub async fn publish(&self, channel: Channel, payload: &[u8]) -> Result<PublisherConfirm, Error>{
        let confirm = retry_on_err!(
            "publish task", 
            channel.basic_publish(
                &self.config.exchange, 
                &self.config.queue, 
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            ).await
        );
        confirm
    }
}