#![allow(dead_code, unused)]
use std::sync::Arc;

use anyhow::{Ok, Result};
use tokio::sync::Mutex;
use tokio_cron_scheduler::JobScheduler;

mod api;
mod broker;
mod config;
mod jobs;

pub type SharedSheduler = Arc<Mutex<JobScheduler>>;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::new();
    cfg.init_tracing();

    let sched = Arc::new(Mutex::new(JobScheduler::new().await?));

    jobs::register_initial_jobs(&mut sched.clone()).await?;

    tracing::info!("Starting scheduler...");
    sched.lock().await.start().await?;

    api::run_server(cfg, sched).await?;
    Ok(())
}
