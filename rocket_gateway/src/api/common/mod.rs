
use rocket_db_pools::{
    deadpool_redis::redis::AsyncCommands, 
    sqlx::{self, Row}, 
    Connection
};

use crate::{Redis, Postgres};

#[get("/healthcheck")]
pub async fn get_healthcheck() -> &'static str {
    "ok"
}

// REDIS USAGE EXAMPLE
#[post("/bo/<id>")]
pub async fn set_bo(mut redis: Connection<Redis>, id: i64) {
    redis.set::<&str, i64, ()>("bo", id).await.expect("cannot set value to redis");
}


// POSTGRES USAGE EXAMPLE
#[get("/test_pg")]
pub async fn test_get_site(mut pg: Connection<Postgres>) -> Option<String> {
    let row: Option<String> = sqlx::query("select version()")
        .fetch_one(&mut **pg)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok();
    row
}