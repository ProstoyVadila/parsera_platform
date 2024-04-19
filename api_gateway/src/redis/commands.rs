use std::error::Error;

use axum::http::StatusCode;
use bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;
use redis::{AsyncCommands, RedisError};

use crate::redis::{RedisConnection,internal_error};


pub async fn get_bo(
    RedisConnection(mut conn): RedisConnection,
) -> Result<String, (StatusCode, String)> {
    let result: String = conn.get("bo").await.map_err(internal_error)?;

    Ok(result)
}

pub async fn set_bo(
    RedisConnection(mut conn): RedisConnection,
) -> &'static str {
    conn.set::<&str, &str, ()>("bo", "bo").await.unwrap();
    "ok"
}

pub async fn set_domain(mut conn: PooledConnection<'static, RedisConnectionManager>, domain: String) {
    conn.set_ex::<String, String, u64>(domain.clone(), domain, 2).await.expect("cannot set domain");
}

pub async fn get_set_domain(mut conn: PooledConnection<'static, RedisConnectionManager>, domain: String) -> Result<String, RedisError> {
    // TODO: figure it out
    let queue = conn.get::<String, String>(domain.clone()).await;
    set_domain(conn, domain).await;
    queue
}