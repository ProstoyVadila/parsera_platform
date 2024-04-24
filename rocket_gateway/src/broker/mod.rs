#![allow(dead_code,unused)]
use std::error::Error;

use rand::{rngs::ThreadRng, seq::SliceRandom};
use rocket::figment::{Figment, providers::Env};
use rocket::serde::Deserialize;
use deadpool_lapin::{
    Config, 
    Manager,
    Object, 
    Pool,
    Runtime,
};
use deadpool_lapin::lapin::{
    protocol::channel,
    types::FieldTable, 
    Channel,
    BasicProperties,
    ExchangeKind,
};
use deadpool_lapin::lapin::options::{
    ExchangeDeclareOptions, 
    QueueBindOptions, 
    QueueDeclareOptions,
    BasicPublishOptions,
};



#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RabbitConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub vhost: String,
    pub exchange: String,
    pub queues: Vec<String>,
}

impl RabbitConfig {
    pub fn get_url(&self) -> String {
        let vhost = {
            if self.vhost.starts_with("/") {
                let mut vhost = self.vhost.clone();
                vhost.replace("/", "%2f");
                vhost
            } else {
                self.vhost.clone()
            }
        };
        format!(
            "amqp://{}:{}@{}:{}/{}", 
            self.user, 
            self.password, 
            self.host, 
            self.port, 
            vhost,
        )
    }
}

pub struct Rabbit {
    pub config: RabbitConfig,
    pub pool: Pool,
}

impl Rabbit {
    pub async fn new() -> Self {
        let config: RabbitConfig = Figment::new()
            .merge(Env::prefixed("RABBITMQ_"))
            .extract()
            .expect("cannot get rabbit config");

        // TODO: add pool config to my rabbit confiig
        let mut pool_config: Config = Config::default();

        pool_config.url = Some(config.get_url());
        let pool = pool_config.create_pool(Some(Runtime::Tokio1)).unwrap();

        let rabbit = Rabbit {
            pool: pool,
            config: config,
        };
        rabbit.declare_all().await;
        rabbit
    }

    pub async fn get_channel(&self) -> Channel {
        let conn = self.pool.get().await.expect("conn err");
        let channel = conn.create_channel().await.expect("channel err");
        channel
    }

    pub async fn declare_all(&self) {
        let channel = self.get_channel().await;
        self.declare_exchange(channel.clone()).await;

        for queue in self.config.queues.clone() {
            self.declare_queue(channel.clone(), queue.clone()).await;
            self.bind_queue(channel.clone(), queue).await;
        }
    }

    pub async fn declare_exchange(&self, channel: Channel) {
        channel.exchange_declare(
            &self.config.exchange, 
            ExchangeKind::Direct,
            ExchangeDeclareOptions::default(), 
            FieldTable::default()
        ).await
         .expect("cannot declare exchange");
    }

    pub async fn declare_queue(&self, channel: Channel, queue: String) {
        channel.queue_declare(
            &queue, 
            QueueDeclareOptions::default(), 
            FieldTable::default()
        ).await
        .expect("cannot declare queue");
    }

    pub async fn bind_queue(&self, channel: Channel, queue: String) {
        channel.queue_bind(
            &queue, 
            &self.config.exchange, 
            &queue, 
            QueueBindOptions::default(), 
            FieldTable::default()
        ).await
         .expect("cannot bind queue");
    }

    fn choose_queue(&self) -> &String {
        if self.config.queues.len() > 1 {
            self.config.queues.choose(&mut rand::thread_rng())
                .expect("cannot get random queue")
        } else {
            &self.config.queues[0]
        }
    }

    pub async fn publish(&self, payload: &[u8]) {
        let channel = self.get_channel().await;
        let queue = self.choose_queue();

        let confirm = channel.basic_publish(
            &self.config.exchange, 
            queue, 
            BasicPublishOptions::default(), 
            payload, 
            BasicProperties::default())
            .await
            .expect("cannot send message to the queue");
    }
}
