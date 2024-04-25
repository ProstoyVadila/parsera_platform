use serde::{Serialize, Deserialize};
use tokio_cron_scheduler::Job;

use crate::SharedSheduler;

#[derive(Debug, Deserialize)]
struct Event {
    name: String,
    rule: String,
    task: String,
}


pub async fn handle_event(sched: SharedSheduler, event: &[u8]) {
    let event: Event = serde_json::from_slice(event).expect("cannot parse event");
    tracing::info!("handling event {:?}", event);

    let job = Job::new(event.rule.as_str(), move |_uuid, _lock| {
        tracing::warn!("{}: {}", event.name, event.task);
    })
    .expect("cannot register a new job for request");

    let job_id = sched
        .lock()
        .await
        .add(job)
        .await
        .expect("cannot register a a new job");
}

