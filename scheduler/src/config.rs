// #![allow(dead_code,unused)]
use std::{
    env,
    net::{SocketAddr, ToSocketAddrs},
};

use envconfig::Envconfig;
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::utils::ParseraService;

pub trait DbAddr {
    fn get_addr(&self) -> String;
}

#[derive(Envconfig, Clone, Debug)]
pub struct DatabaseConfig {
    #[envconfig(from = "POSTGRES_HOST", default = "localhost")]
    pub host: String,
    #[envconfig(from = "POSTGRES_PORT", default = "5432")]
    pub port: u16,
    #[envconfig(from = "POSTGRES_PASSWORD", default = "")]
    pub password: String,
    #[envconfig(from = "POSTGRES_DB", default = "postgres")]
    pub db: String,
    #[envconfig(from = "POSTGRES_USER", default = "postgres")]
    pub user: String,
}

impl DbAddr for DatabaseConfig {
    fn get_addr(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db
        )
    }
}

#[derive(Envconfig, Clone, Debug)]
pub struct BrokerConfig {
    #[envconfig(from = "RABBITMQ_HOST")]
    pub host: String,
    #[envconfig(from = "RABBITMQ_PORT")]
    pub port: u16,
    #[envconfig(from = "RABBITMQ_USER")]
    pub user: String,
    #[envconfig(from = "RABBITMQ_PASSWORD")]
    pub password: String,
    #[envconfig(from = "RABBITMQ_VHOST")]
    pub vhost: String,
    #[envconfig(from = "RABBITMQ_POOL_MAX_SIZE", default = "16")]
    pub pool_max_size: u16,
    #[envconfig(from = "RABBITMQ_POOL_TIMEOUT", default = "2")]
    pub pool_timeout: u16,
    #[envconfig(from = "RABBITMQ_CONSUME_EXCHANGE", default = "to_scheduler")]
    pub consume_exchange: String,
    #[envconfig(from = "RABBITMQ_PRODUCE_EXCHANGE", default = "from_scheduler")]
    pub produce_exchange: String,
    #[envconfig(from = "RABBITMQ_CONSUME_QUEUE", default = "to_scheduler")]
    pub consume_queue: String,
    #[envconfig(from = "RABBITMQ_SCRAPER_QUEUE")]
    pub scraper_queue: String,
    #[envconfig(from = "RABBITMQ_HEAVY_ARTILLERY_QUEUE")]
    pub heavy_artillery_queue: String,
    #[envconfig(from = "RABBITMQ_EXTRACTOR_QUEUE")]
    pub extractor_queue: String,
    #[envconfig(from = "RABBITMQ_NOTIFICATION_QUEUE")]
    pub notification_queue: String,
    #[envconfig(from = "RABBITMQ_DB_MANAGER_QUEUE")]
    pub db_manager_queue: String,
    #[envconfig(from = "RABBITMQ_STATUS_MANAGER_QUEUE")]
    pub status_manager_queue: String,
}

impl BrokerConfig {
    // TODO: remove this mock
    pub fn queues_to_produce(&self) -> Vec<&str> {
        // self.produce_queues.split(',').map(|q| q.to_string()).collect()
        vec![
            &self.scraper_queue,
            &self.heavy_artillery_queue,
            &self.extractor_queue,
            &self.notification_queue,
            &self.db_manager_queue,
            &self.status_manager_queue,
        ]
    }

    pub fn queue_to_consume(&self) -> String {
        self.consume_queue.clone()
    }
}

impl DbAddr for BrokerConfig {
    fn get_addr(&self) -> String {
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
            self.user, self.password, self.host, self.port, vhost
        )
    }
}

#[derive(Envconfig, Clone, Debug)]
pub struct Config {
    #[envconfig(nested = true)]
    pub database: DatabaseConfig,
    #[envconfig(nested = true)]
    pub broker: BrokerConfig,
    #[envconfig(from = "LOG_FORMAT", default = "text")]
    pub log_format: String,
    #[envconfig(from = "HOST", default = "localhost")]
    pub host: String,
    #[envconfig(from = "PORT", default = "8080")]
    pub port: u16,
}

impl Config {
    pub fn new() -> Config {
        // If LOCAL_RUN is set and true, load .env file
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "debug")
        }
        let log_format = match env::var("LOG_FORMAT") {
            Ok(l) => l,
            _ => "text".into(),
        };
        Self::init_tracing(log_format.as_str());

        if env::var("LOCAL_RUN").is_ok_and(|local| local == "true") {
            use dotenv::dotenv;

            dotenv().ok();
            tracing::info!("Local Run mode enabled")
        }

        match Config::init_from_env() {
            Ok(config) => config,
            Err(err) => {
                tracing::error!("Cannot load config: {}", err);
                panic!("cannot load config");
            }
        }
    }

    pub fn init_tracing(log_format: &str) {
        if log_format == "json" {
            tracing_subscriber::registry()
                .with(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "scheduler=debug".into()),
                )
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        } else {
            tracing_subscriber::registry()
                .with(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "scheduler=debug".into()),
                )
                .with(tracing_subscriber::fmt::layer())
                .init();
        }
    }

    pub fn get_socket_addr(&self) -> SocketAddr {
        let default_socket_addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        if self.host == "localhost" {
            return default_socket_addr;
        }
        let addr = match (self.host.clone(), self.port).to_socket_addrs() {
            Ok(mut addr_iter) => addr_iter.next(),
            Err(err) => {
                tracing::error!("cannot resolve socket address: {}", err);
                tracing::warn!("setting default addr: {}", default_socket_addr);
                Some(default_socket_addr)
            }
        };
        match addr {
            Some(addr) => addr,
            _ => default_socket_addr,
        }
    }
}
