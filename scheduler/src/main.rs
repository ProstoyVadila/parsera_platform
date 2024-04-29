#![allow(dead_code, unused)]
use std::sync::Arc;

use anyhow::{Ok, Result};
use tokio::sync::Mutex;
use tokio_cron_scheduler::JobScheduler;

mod api;
mod broker;
mod config;
mod jobs;
mod scheduler;
mod utils;

pub type SharedSheduler = Arc<Mutex<JobScheduler>>;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::new();

    
    let sched = Arc::new(Mutex::new(JobScheduler::new().await?));
    
    jobs::register_initial_jobs(&mut sched.clone()).await?;
    
    tracing::info!("Connecting to rabbitmq");
    tracing::info!("Starting scheduler...");
    sched.lock().await.start().await?;

    let rabbit = broker::Rabbit::new(cfg.broker.clone()).await?;
    let sched_cloned = sched.clone();
    tokio::spawn(async move {
        rabbit.consume(sched_cloned).await.expect("error in consumer");
    });

    api::run_server(cfg, sched).await?;
    Ok(())
}
