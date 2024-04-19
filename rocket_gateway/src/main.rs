// #![allow(dead_code, unused)]
#[macro_use] extern crate rocket;

use rocket_db_pools::{deadpool_redis, sqlx, Database};

mod api;

#[derive(Database)]
#[database("redis")]
struct Redis(deadpool_redis::Pool);

#[derive(Database)]
#[database("postgres")]
struct Postgres(sqlx::PgPool);


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .attach(Redis::init())
        .attach(Postgres::init())
        .mount("/", api::get_routes())
        .launch()
        .await?;

    Ok(())
}