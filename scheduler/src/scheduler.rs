use serde::{Serialize, Deserialize};
use tokio_cron_scheduler::Job;

use crate::{broker::Rabbit, SharedSheduler};

#[derive(Debug, Deserialize)]
struct Event {
    name: String,
    rule: String,
    task: String,
}


pub async fn handle_event(broker: &Rabbit, sched: SharedSheduler, event: &[u8]) {
    let event_job: Event = serde_json::from_slice(event).expect("cannot parse event");
    tracing::info!("handling event {:?}", event_job);

    let job = Job::new(event_job.rule.as_str(), move |_uuid, _lock| {
        tracing::warn!("{}: {}", event_job.name, event_job.task);
    })
    .expect("cannot register a new job for request");

    let job_id = sched
        .lock()
        .await
        .add(job)
        .await
        .expect("cannot register a a new job");

    tracing::debug!("publishing command to queue");
    broker.publish(event, crate::utils::ParseraService::Notification).await;
}

