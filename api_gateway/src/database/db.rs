use std::time::Duration;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};

use sqlx::{
    pool::PoolConnection, postgres::{PgPool, PgPoolOptions}, Error, Pool as PostgresPool, Postgres
};


use crate::retry_inc_on_err;


pub async fn try_get_pg_pool(db_addr: String) -> PostgresPool<Postgres> {
    let pg_pool = retry_inc_on_err!("get_pg_pool", get_pg_pool(db_addr.clone()).await);
    let pg_pool = match pg_pool {
        Ok(pool) => pool,
        Err(err) => {
            tracing::error!("cannot establish connection with database: {}", err);
            // TODO: exit?
            panic!("cannot establish connection to database");
        }
    };
    tracing::info!("establish connection with database");
    pg_pool
}



pub async fn get_pg_pool(db_addr: String) -> Result<PostgresPool<Postgres>, Error> {
    tracing::debug!("pg url: {}", db_addr);
    let res = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_addr)
        .await;
    res
}


// pub async fn using_connection_extractor(
//     DatabaseConnection(mut conn): DatabaseConnection,
// ) -> Result<String, (StatusCode, String)> {
//     sqlx::query_scalar("select 'hello world from pg'")
//         .fetch_one(&mut *conn)
//         .await
//         .map_err(internal_error)
// }
