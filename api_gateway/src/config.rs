use std::{env, net::{SocketAddr, ToSocketAddrs}};

use envconfig::Envconfig;
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub trait DbAddr {
    fn get_addr(&self) -> String;
}

#[derive(Envconfig, Clone, Debug)]
pub struct Database {
    #[envconfig(from = "POSTGRES_HOST", default = "localhost")]
    pub host: String,
    #[envconfig(from = "POSTGRES_PORT", default = "5432")]
    pub port: u16,
    #[envconfig(from = "POSTGRES_PASSWORD", default = "" )]
    pub password: String,
    #[envconfig(from = "POSTGRES_DB", default = "postgres")]
    pub db: String,
    #[envconfig(from = "POSTGRES_USER", default = "postgres")]
    pub user: String
}

impl DbAddr for Database {
    fn get_addr(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.db)
    }
}

#[derive(Envconfig, Clone, Debug)]
pub struct RedisConfig {
    #[envconfig(from = "REDIS_HOST", default = "localhost")]
    pub host: String,
    #[envconfig(from = "REDIS_PORT", default = "6379")]
    pub port: u16,
    #[envconfig(from = "REDIS_PASSWORD", default = "password")]
    pub password: String,
    #[envconfig(from = "REDIS_DB", default = "")]
    pub db: String,
    #[envconfig(from = "REDIS_USER", default = "default")]
    pub user: String,
}

impl DbAddr for RedisConfig {
    fn get_addr(&self) -> String {
        tracing::debug!("GET DSN: {} {}", self.user, self.host);
        format!("redis://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.db)
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
    #[envconfig(from = "RABBITMQ_EXCHANGE")]
    pub exchange: String,
    #[envconfig(from = "RABBITMQ_QUEUE")]
    pub queue: String,
    #[envconfig(from = "RABBITMQ_QUEUE_NUM")]
    pub queue_num: u8,
}

impl DbAddr for BrokerConfig {
    fn get_addr(&self) -> String {
        let vhost = {
            if self.vhost.starts_with("/") {
                let mut vhost = self.vhost.clone();
                vhost.remove(0);
                vhost
            } else {
                self.vhost.clone()
            }

        };
        format!("amqp://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, vhost)
    }
}

#[derive(Envconfig, Clone, Debug)]
pub struct Config {
    #[envconfig(nested = true)]
    pub database: Database,
    #[envconfig(nested = true)]
    pub cache: RedisConfig,
    #[envconfig(nested = true)]
    pub broker: BrokerConfig,
    #[envconfig(from = "LOG_FORMAT", default="text")]
    pub log_format: String,
    #[envconfig(from = "HOST", default="localhost")]
    pub host: String,
    #[envconfig(from = "PORT", default="8080")]
    pub port: u16,
}

impl Config {
    pub fn get_socket_addr(&self) -> SocketAddr {
        let default_socket_addr = SocketAddr::from(([127,0,0,1], 8080));
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
        let addr = match addr {
            Some(addr) => addr,
            _ => default_socket_addr,
        };
        addr
    }
}

impl Config {
    pub fn new() -> Config {
        // If LOCAL_RUN is set and true, load .env file
        if env::var("LOCAL_RUN").is_ok_and(|local| local == "true") {
            use dotenv::dotenv;

            dotenv().ok();
            tracing::info!("Local Run mode enabled")
        }

        let config = match Config::init_from_env() {
            Ok(config) => config,
            Err(err) => {
                tracing::error!("Cannot load config: {}", err);
                panic!("cannot load config");
            }
        };
        config
    }

    pub fn set_tracing(&self) {
        if self.log_format == "json" {
            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "api_gateway=debug,axum=debug,tower_http=debug".into()))
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        } else {
            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "api_gateway=debug,axum=debug,tower_http=debug".into()))
                .with(tracing_subscriber::fmt::layer())
                .init();
        }
    }
}
