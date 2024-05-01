use anyhow::Result;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::SharedSheduler;

fn initial_job() -> Result<Job, JobSchedulerError> {
    Job::new("1/10 * * * * *", |_uuid, _lock| {
        tracing::warn!("I am initial job!");
    })
}

pub async fn register_initial_jobs(mut sched: &mut SharedSheduler) -> Result<()> {
    tracing::info!("Registering initial jobs for scheduler");
    sched
        .lock()
        .await
        .add(initial_job().expect("cannot register initial job"))
        .await?;
    Ok(())
}
