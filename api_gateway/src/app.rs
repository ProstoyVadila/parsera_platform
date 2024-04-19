//#![allow(dead_code,unused)]
use std::{time::Duration, sync::Arc};

use axum::Router;
use tower_http::trace::{self, TraceLayer};
use tower_http::timeout::TimeoutLayer;
use tracing::Level;

use crate::{
    rabbit::RabbitMQ,
    config::{BrokerConfig, Config, Database, DbAddr, RedisConfig},
    // routes::set_routes
};
use crate::redis::{self, RedisConnectionPool};
use crate::{database, routes};


pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub rabbit: RabbitMQ,
    pub pg_pool: sqlx::Pool<sqlx::Postgres>,
    pub redis_pool: RedisConnectionPool,
}

pub struct App {
    pub router: Router,
    pub config: Config,
}

impl App {
    pub async fn new(cfg: Config) -> App {
        let pg_pool = Self::get_pg_pool(cfg.database.clone()).await;
        let redis_pool = Self::get_redis_pool(cfg.cache.clone()).await;
        let rabbit = Self::get_rabbitmq(cfg.broker.clone()).await;

        let shared_state = Arc::new(AppState{rabbit, pg_pool, redis_pool});

        let router = Router::new()
            .nest("/", routes::api())
            // .route("/healthcheck", get(handlers::healthcheck))
            // .route("/smth", post(handlers::add_site))
            .with_state(shared_state)
            // .with_state(redis_pool)
            .layer((
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
                TimeoutLayer::new(Duration::from_secs(10)),
            ));

        // let router = set_routes(router);
        App {
            router,
            config: cfg,
        }
    }


    async fn get_pg_pool(cfg: Database) -> sqlx::Pool<sqlx::Postgres> {
        tracing::info!("Setting up postgres pool...");
        let pg_pool = database::try_get_pg_pool(cfg.get_addr()).await;
        pg_pool
    }


    async fn get_redis_pool(cfg: RedisConfig) -> RedisConnectionPool {
        tracing::info!("Setting up redis pool...");
        let redis_pool = redis::get_redis_pool(cfg).await;
        redis_pool
    }

    async fn get_rabbitmq(cfg: BrokerConfig) -> RabbitMQ {
        tracing::info!("Setting up rabbitmq pool...");
        let rabbit_pool = RabbitMQ::new(cfg).await;
        rabbit_pool
    }
}
