// #![allow(dead_code, unused)]
#[macro_use] extern crate rocket;

use std::env;

use rocket_db_pools::{deadpool_redis, sqlx, Database};

use broker::Rabbit;

mod api;
mod broker;


#[derive(Database)]
#[database("redis")]
struct Redis(deadpool_redis::Pool);

#[derive(Database)]
#[database("postgres")]
struct Postgres(sqlx::PgPool);


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    if env::var("LOCAL_RUN").is_ok_and(|local| local == "true") {
        use dotenv::dotenv;
        dotenv().ok();
    }

    let rabbit = Rabbit::new().await;
    
    let _ = rocket::build()
        .attach(Redis::init())
        .attach(Postgres::init())
        .mount("/", api::get_routes())
        .manage(rabbit)
        .launch()
        .await?;

    Ok(())
}