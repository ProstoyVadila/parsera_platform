#![allow(dead_code,unused)]
use std::process::exit;

use bb8::{ManageConnection, Pool, PooledConnection};
use bb8_lapin::{lapin, lapin::ConnectionProperties, LapinConnectionManager};
use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions};
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
        broker.declare_queue(channel.clone()).await;
        broker.bind_queue(channel).await;
        broker
    }

    async fn create_rabbitmq_pool(addr: String) -> RabbitPool{
        let conn_props = ConnectionProperties::default();
        let manager = LapinConnectionManager::new(&addr, conn_props);
        let pool = Pool::builder()
            .max_size(16)
            .build(manager)
            .await
            .unwrap();
        pool
    }

    pub async fn try_conn(&self) -> RabbitConn {
        let conn = match retry_inc_on_err!("try rabbitmq conn", self.pool.get().await) {
            Ok(conn) => conn,
            Err(err) => {
                tracing::error!("cannot connect to rabbitmq: {}", err);
                // TODO: fix this after tests
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
        let opts = ExchangeDeclareOptions{
            durable: true,  // non default, recreate exchange on restart 
            ..Default::default()
        };
        channel.exchange_declare(
            &self.config.exchange,
            ExchangeKind::Direct,
            opts,
            FieldTable::default()
        ).await
         .expect("cannot declare exchange");
    }

    async fn declare_queue(&self, channel: Channel) {
        // let _ = channel.queue_bind(queue, exchange, routing_key, options, arguments)
        let opts = QueueDeclareOptions{
            durable: true,  // non default, recreate queue on restart
            ..Default::default()
        };
        channel.queue_declare(
            &self.config.queue, 
            opts, 
            FieldTable::default()
        ).await
         .expect("cannot declare a queue");
    }

    async fn bind_queue(&self, channel: Channel) {
        channel.queue_bind(
            &self.config.queue,
            &self.config.exchange, 
            &self.config.queue, 
            QueueBindOptions::default(),
            FieldTable::default(),
        ).await;
    }

    pub async fn publish(&self, payload: &[u8]) -> Result<PublisherConfirm, Error>{
        let opts = BasicPublishOptions{
            mandatory: true, //  non default, return msg back if the msg is unroutable https://www.rabbitmq.com/amqp-0-9-1-reference#basic.publish.mandatory
            immediate: false,
        };
        let props = BasicProperties::default().with_delivery_mode(2); // persistent messages
        let channel = match self.get_channel().await {
            Ok(ch) => ch,
            Err(err) => {
                tracing::error!("cannot get channel: {}", err);
                return Err(err)
            }
        };
        let confirm = retry_on_err!(
            "publish task", 
            channel.basic_publish(
                &self.config.exchange, 
                &self.config.queue, 
                opts,
                payload,
                BasicProperties::default(),
            ).await
        );
        confirm
    }
}