use axum::http::StatusCode;
use redis::AsyncCommands;

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
