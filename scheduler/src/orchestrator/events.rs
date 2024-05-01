use anyhow::Result;
use deadpool_lapin::lapin::publisher_confirm::PublisherConfirm;
use serde::{Serialize, Deserialize};
use tokio_cron_scheduler::Job;

use common::models::{
    EventCommandStatus, Crawler, EventProtocol, EventProtocolData, Page, Priority, EventCommand, Site
};
use uuid::Uuid;

use crate::{broker::Rabbit, config::DatabaseConfig, database::Postgres, utils::ParseraService, SharedSheduler};

// #[derive(Debug, Deserialize)]
// struct Event {
//     name: String,
//     rule: String,
//     task: String,
// }
//
//
// pub async fn handle_event(broker: &Rabbit, sched: SharedSheduler, event: &[u8]) {
//     let event_job: Event = match serde_json::from_slice(event);
//     tracing::info!("handling event {:?}", event_job);

//     let job = Job::new(event_job.rule.as_str(), move |_uuid, _lock| {
//         tracing::warn!("{}: {}", event_job.name, event_job.task);
//     })
//     .expect("cannot register a new job for request");

//     let job_id = sched
//         .lock()
//         .await
//         .add(job)
//         .await
//         .expect("cannot register a a new job");

//     tracing::debug!("publishing command to queue");
//     let confirm = match broker.publish(event, crate::utils::ParseraService::Notification).await {
//         Ok(c) => c,
//         Err(err) => {
//             tracing::error!("cannot send an event to queue: {}", err);
//             return;
//         }
//     };
// }

pub async fn handle_event(broker: &Rabbit, sched: SharedSheduler, msg: &[u8]) {
    let event: EventProtocol = match serde_json::from_slice(msg) {
        Ok(e) => e,
        Err(err) => {
            let msg = std::str::from_utf8(msg).expect("invalid message slice");
            tracing::error!("Error trying handle a new message in consumer. Msg: {}, Err: {}", msg, err);
            return;
        },
    };

    match event.command.clone() {
        EventCommand::RegisterCrawler(_) => handle_register_crawler(broker, event).await,
        EventCommand::ScrapePage(status) => handle_scrape(broker, status, event).await,
        EventCommand::ExtractPage(status) => handle_extraction(broker, status, event).await,
        EventCommand::StorePage(status) => handle_store(broker, status, event).await,
        EventCommand::NotifyUser(status) => handle_notification(broker, status, event).await,
        EventCommand::Sleep(status) => handle_sleep(broker, status, event).await,
    };
}

pub async fn handle_register_crawler(broker: &Rabbit, event: EventProtocol) {
    let db = Postgres::new(DatabaseConfig{
        host: "localhost".to_string(),
        port: 5432,
        password: "pass".to_string(),
        db: "crawlers".to_string(),
        user: "postgres".to_string(),
    });
    let crawler = match event.data {
        EventProtocolData::External(crawler) => crawler,
        EventProtocolData::Internal(_) => {
            tracing::error!("got a register crawler command but a message format is internal.");
            return;
        },
    };
    let page = Page {
        id: Uuid::now_v7(),
        crawler_id: crawler.id,
        site_id: crawler.site.id,
        url: crawler.site.start_page,
        domain: crawler.site.domain,
        is_pagination: false,
        times_reparsed: 0,
        priority: crawler.priority,
        notification: crawler.notification,
        xpaths: crawler.site.page_xpaths,
        created_at: crawler.created_at,
        updated_at: crawler.updated_at,
        html: None,
        data: None,
        meta: crawler.meta,
    };
    let scrape_event = EventProtocol {
        command: EventCommand::ScrapePage(EventCommandStatus::Pending),
        data: EventProtocolData::Internal(page),
    };
    handle_scrape(broker, EventCommandStatus::Pending, scrape_event).await;
}

pub async fn handle_scrape(broker: &Rabbit, status: EventCommandStatus, event: EventProtocol) {
    // TODO change status of event
    let mut msg = match serde_json::to_string(&event) {
        Ok(msg) => msg,
        Err(err) => {
            tracing::error!("cannot serialize event got: {}", err);
            return;
        }
    };
    match status {
        EventCommandStatus::Pending => {
            broker.publish(msg.as_bytes(), ParseraService::Scraper).await;
        },
        EventCommandStatus::Done => {
            broker.publish(msg.as_bytes(), ParseraService::Extractor).await;
        },
        EventCommandStatus::Failed => {
            broker.publish(msg.as_bytes(), ParseraService::HeavyArtillery).await;
        },
    }
}

pub async fn handle_extraction(broker: &Rabbit, status: EventCommandStatus, event: EventProtocol) {
    // TODO change status of event
    let msg = match serde_json::to_string(&event) {
        Ok(msg) => msg,
        Err(err) => {
            tracing::error!("cannot serialize event got: {}", err);
            return;
        }
    };
    match status {
        EventCommandStatus::Pending => {
            // broker.publish(payload, to)
            tracing::warn!("Got extraction message with pending status");
            return;
        },
        EventCommandStatus::Done => {
            broker.publish(msg.as_bytes(), ParseraService::DatabaseManager).await;
            // TODO: check notification rule
            broker.publish(msg.as_bytes(), ParseraService::Notification).await;
        },
        EventCommandStatus::Failed => {
            tracing::warn!("Got failed job from extractor. Store + Notification");
            broker.publish(msg.as_bytes(), ParseraService::DatabaseManager).await;
            broker.publish(msg.as_bytes(), ParseraService::Notification).await;
        },
    }
}

pub async fn handle_store(broker: &Rabbit, status: EventCommandStatus, event: EventProtocol) {
    match status {
        EventCommandStatus::Pending => todo!(),
        EventCommandStatus::Done => todo!(),
        EventCommandStatus::Failed => todo!(),
    }
}

pub async fn handle_notification(broker: &Rabbit, status: EventCommandStatus, event: EventProtocol) {
    match status {
        EventCommandStatus::Pending => todo!(),
        EventCommandStatus::Done => todo!(),
        EventCommandStatus::Failed => todo!(),
    }
}

pub async fn handle_sleep(broker: &Rabbit, status: EventCommandStatus, event: EventProtocol) {
    match status {
        EventCommandStatus::Pending => todo!(),
        EventCommandStatus::Done => todo!(),
        EventCommandStatus::Failed => todo!(),
    }
}
