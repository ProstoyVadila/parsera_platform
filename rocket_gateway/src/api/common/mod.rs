use std::time::Duration;
use rocket::tokio::{sync::oneshot, time::sleep};
use rocket::{tokio, State};

use rocket_db_pools::{
    deadpool_redis::redis::AsyncCommands, 
    sqlx::{self, Row}, 
    Connection
};

use crate::broker::Rabbit;
use crate::{Postgres, Redis};

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

// test with state
#[get("/test_state_rabbit")]
pub async fn test_state_rabbit(rabbit: &State<Rabbit>) -> &'static str {
    rabbit.publish("hello there".as_bytes()).await;
    "ok"
}

#[get("/test_spawn_task")]
pub async fn get_spawn_task() -> String {
    println!("Creating oneshot");
    let (tx, rx) = oneshot::channel::<String>();
    tokio::spawn(async move {
        println!("going to sleep 3 secs");
        sleep(Duration::from_secs(3)).await;
        println!("Sending message");
        tx.send("I slept a lot".into()).expect("cannot send oneshot msg to rx");
    });

    println!("Waiting for msg");
    let msg = rx.await.expect("cannot get oneshot msg");
    println!("Got the msg!");
    msg
}