#![allow(dead_code,unused)]
use std::process::exit;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use bb8_redis::bb8;

use crate::config::{RedisConfig, DbAddr};
use crate::utils::retry_inc_on_err;

mod commands;
pub use commands::*;

pub struct RedisConnection(PooledConnection<'static, RedisConnectionManager>);
pub type RedisConnectionPool = Pool<RedisConnectionManager>;


pub async fn get_redis_pool(cfg: RedisConfig) -> RedisConnectionPool {
    tracing::debug!("redis config: {}", cfg.get_addr());
    let manager = match RedisConnectionManager::new(cfg.get_addr()) {
        Ok(m) => m,
        Err(err) => {
            tracing::error!("cannot get redis connection manager: {}", err);
            // TODO: exit?
            exit(1);
        }
    };
    let pool = retry_inc_on_err!(
        "get_redis_pool", 
        bb8::Pool::builder().build(manager.clone()).await
    );
    let pool = match pool {
        Ok(p) => p,
        Err(err) => {
            tracing::error!("cannot get redis pool: {}", err);
            // TODO: exit?
            exit(1);
        }
    };
    check_conn(pool.clone()).await;
    pool
}

pub async fn check_conn(pool: RedisConnectionPool) {
    let mut conn = pool.get().await.unwrap();
    conn.set_ex::<&str, &str, ()>("foo", "bar", 3).await.unwrap();
    let result: String = conn.get("foo").await.unwrap();
    assert_eq!(result, "bar");
}

// pub async fn get_RedisConnection()

#[async_trait]
impl<S> FromRequestParts<S> for RedisConnection
where
    RedisConnectionPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = RedisConnectionPool::from_ref(state);

        // TODO: fix retry_on_err
        // let conn = retry_on_err!("get_redis_pool", pool.get_owned().await, 2, 0.1);
        // TODO: fix on disconnect (want 500 internal error, got 408 request timeout) 
        let conn = pool.get_owned().await;
        let conn = conn.map_err(internal_error)?;

        Ok(Self(conn))
    }
}


fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("got error: {}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
