#![allow(dead_code,unused)]
use std::collections::HashSet;
use std::{
    iter::repeat,
    process::exit,
};

use bb8::{ManageConnection, Pool, PooledConnection};
use bb8_lapin::{lapin, lapin::ConnectionProperties, LapinConnectionManager};
use bb8_redis::RedisConnectionManager;
use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions};
use lapin::protocol::channel;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Error, ExchangeKind};
use lapin::publisher_confirm::PublisherConfirm;
use tracing::event;
use rand::{thread_rng, seq::SliceRandom};

use common::models::AddSiteEvent;

use crate::config::{BrokerConfig as BrokerConfig, DbAddr};
use crate::redis::{RedisConnection, get_set_domain};
use crate::utils::{retry_inc_on_err, retry_on_err};


pub type RabbitConn<'a> = PooledConnection<'a, LapinConnectionManager>;
pub type RabbitPool = Pool<LapinConnectionManager>;


pub struct RabbitMQ {
    pool: RabbitPool,
    config: BrokerConfig,
    queues: Vec<String>,
}

impl RabbitMQ {
    pub async fn new(config: BrokerConfig) -> Self {
        let pool = Self::create_rabbitmq_pool(config.get_addr()).await;

        // TODO: add prefix to env
        let queue_num = config.queue_num.clone();
        let queues = Self::get_queues("queue".to_string(), queue_num);
        tracing::info!("Queues: {:?}", queues);
    
        let broker = RabbitMQ { pool , config, queues };

        let channel = broker.get_channel().await.unwrap();
        broker.declare_all(channel).await;

        broker
    }

    fn get_queues(prefix: String, queue_num: u8) -> Vec<String> {
        Vec::from_iter(
            repeat(queue_num)
            .take(queue_num.into())
            .enumerate()
            .map(|(i, _)| format!("{}{}", prefix, i+1))
        )
    }

    fn get_random_queue(&self) -> &str {
        let mut rng = rand::thread_rng();
        self.queues.choose(&mut rng).unwrap()
    }

    async fn create_rabbitmq_pool(addr: String) -> RabbitPool{
        // TODO: figure out with shuffling conns in pool
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
        // TODO: configure timeout and step for retry
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
        // TODO: configure timeout and step for retry
        let channel = match retry_on_err!("try rabbitmq channel", conn.create_channel().await) {
            Ok(ch) => ch,
            Err(err) => {
                tracing::error!("cannot get a rabbitmq channel: {}", err);
                return Err(err);
            }
        };
        Ok(channel)
    }

    async fn declare_all(&self, channel: Channel) {
        self.declare_exchange(channel.clone()).await;
        for queue in self.queues.clone() {
            tracing::info!("declaring queue {}", queue);
            self.declare_queue(channel.clone(), queue.clone()).await;
            self.bind_queue(channel.clone(), queue).await;
        }
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

    async fn declare_queue(&self, channel: Channel, queue: String) {
        let opts = QueueDeclareOptions{
            durable: true,  // non default, recreate queue on restart
            ..Default::default()
        };
        channel.queue_declare(
            &queue, 
            opts, 
            FieldTable::default()
        ).await
         .expect("cannot declare a queue");
    }

    async fn bind_queue(&self, channel: Channel, queue: String) {
        channel.queue_bind(
            // &self.config.queue,
            &queue,
            &self.config.exchange, 
            &queue, 
            QueueBindOptions::default(),
            FieldTable::default(),
        ).await;
    }

    pub async fn publish(&self, payload: &[u8]) -> Result<PublisherConfirm, Error> {
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
        let queue = self.get_random_queue();
        let confirm = retry_on_err!(
            "basic_publish", 
            channel.basic_publish(
                &self.config.exchange, 
                &queue, 
                opts,
                payload,
                BasicProperties::default(),
            ).await
        );
        confirm
    }

    async fn publish_to_queue(&self, queue: &str, payload: &[u8]) -> Result<PublisherConfirm, Error> {
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
            "basic_publish", 
            channel.basic_publish(
                &self.config.exchange, 
                queue, 
                opts,
                payload,
                BasicProperties::default(),
            ).await
        );
        confirm 
    }

    pub async fn rate_publish(&self, redis_conn: PooledConnection<'static, RedisConnectionManager>, domain: String, payload: &[u8]) -> Result<PublisherConfirm, Error> {
        let mut rng = rand::thread_rng();
        
        let queues = match get_set_domain(redis_conn, domain).await {
            Ok(queue) => {
                let mut queues_cloned = self.queues.clone();
                queues_cloned.retain(|x| *x != queue);
                queues_cloned
            }
            _ => self.queues.clone()
        };

        let to_queue = match queues.choose(&mut rng) {
            Some(q) => q,
            _ => {
                tracing::error!("cannot get a value from choose from queue");
                queues.get(0).unwrap()
            }
        };

        self.publish_to_queue(to_queue, payload).await
    }
}