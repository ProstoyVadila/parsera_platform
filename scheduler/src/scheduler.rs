use serde::{Serialize, Deserialize};
use tokio_cron_scheduler::Job;

use common::models::{
    CommandStatus,
    Crawler,
    SchedulerCommand,
    Page,
    Priority,
    Site,
    MessageToScheduler,
};

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

pub async fn handle_msg(broker: &Rabbit, sched: SharedSheduler, msg: &[u8]) {
    let event: MessageToScheduler = match serde_json::from_slice(msg) {
        Ok(e) => e,
        Err(err) => {
            let msg = std::str::from_utf8(msg).expect("invalid message slice");
            tracing::error!("Error trying handle a new message in consumer. Msg: {}, Err: {}", msg, err);
            return;
        },
    };

    let action = event.action.clone();
    match action {
        SchedulerCommand::RegisterCrawler(_) => {
            if let Some(ref crawler) = event.crawler {
                // TODO: add register crawler
                handle_register_crawler(broker, event).await;
            } else {
                tracing::error!("there is no new crawler for register command")
            }
        },
        SchedulerCommand::ScrapePage(status) => handle_scrape(broker, status, event).await,
        SchedulerCommand::ExtractPage(status) => handle_extraction(broker, status, event).await,
        SchedulerCommand::StorePage(status) => handle_store(broker, status, event).await,
        SchedulerCommand::NotifyUser(status) => handle_notification(broker, status, event).await,
        SchedulerCommand::Sleep(status) => handle_sleep(broker, status, event).await,
    };
}

pub async fn handle_register_crawler(broker: &Rabbit, event: MessageToScheduler) {
    todo!()
}

pub async fn handle_scrape(broker: &Rabbit, status: CommandStatus, event: MessageToScheduler) {
    match status {
        CommandStatus::Pending => todo!(),
        CommandStatus::Done => todo!(),
        CommandStatus::Failed => todo!(),
    }
}

pub async fn handle_extraction(broker: &Rabbit, status: CommandStatus, event: MessageToScheduler) {
    match status {
        CommandStatus::Pending => todo!(),
        CommandStatus::Done => todo!(),
        CommandStatus::Failed => todo!(),
    }
}

pub async fn handle_store(broker: &Rabbit, status: CommandStatus, event: MessageToScheduler) {
    match status {
        CommandStatus::Pending => todo!(),
        CommandStatus::Done => todo!(),
        CommandStatus::Failed => todo!(),
    }
}

pub async fn handle_notification(broker: &Rabbit, status: CommandStatus, event: MessageToScheduler) {
    match status {
        CommandStatus::Pending => todo!(),
        CommandStatus::Done => todo!(),
        CommandStatus::Failed => todo!(),
    }
}

pub async fn handle_sleep(broker: &Rabbit, status: CommandStatus, event: MessageToScheduler) {
    match status {
        CommandStatus::Pending => todo!(),
        CommandStatus::Done => todo!(),
        CommandStatus::Failed => todo!(),
    }
}
