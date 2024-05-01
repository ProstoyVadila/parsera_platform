#![allow(dead_code, unused)]
use std::sync::Arc;

use anyhow::{Ok, Result};
use tokio::sync::Mutex;
use tokio_cron_scheduler::{JobScheduler, PostgresMetadataStore, PostgresNotificationStore, SimpleJobCode, SimpleNotificationCode};

use common::infinite_retry;

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

    
    // let metadata_storage = Box::new(PostgresMetadataStore::default());
    // let notification_storage = Box::new(PostgresNotificationStore::default());
    // let simple_job_code = Box::new(SimpleJobCode::default());
    // let simple_notification_code = Box::new(SimpleNotificationCode::default());

    // let mut job_sched = JobScheduler::new_with_storage_and_code(
    //     metadata_storage,
    //     notification_storage,
    //     simple_job_code,
    //     simple_notification_code
    // ).await?;

    // job_sched.shutdown_on_ctrl_c();
    // job_sched.set_shutdown_handler(Box::new(|| {
    //     Box::pin(async move {
    //         tracing::info!("JobScheduler stoped.")
    //     })
    // }));
    // let sched = Arc::new(Mutex::new(job_sched));
    let sched = Arc::new(Mutex::new(JobScheduler::new().await?));
    
    jobs::register_initial_jobs(&mut sched.clone()).await?;
    
    tracing::info!("Connecting to rabbitmq");
    tracing::info!("Starting scheduler...");
    sched.lock().await.start().await?;

    let rabbit = broker::Rabbit::new(cfg.broker.clone()).await?;
    let sched_cloned = sched.clone();
    tokio::spawn(async move {
        infinite_retry!("broker consumer", rabbit.consume(sched_cloned.clone()).await);
    });

    api::run_server(cfg, sched).await?;
    Ok(())
}
