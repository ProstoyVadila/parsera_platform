#![allow(dead_code, unused)]
use std::sync::{Arc, Mutex};

use anyhow::{Ok, Result};
use tokio_cron_scheduler::JobScheduler;

mod api;
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
    sched
        .lock()
        .expect("cannot start scheduler")
        .start()
        .await?;

    api::run_server(cfg, sched).await?;
    Ok(())
}
