use std::{collections::HashMap, hash::Hash};

use anyhow::{Ok, Result};

use chrono::{DateTime, Utc};
use common::models::{Crawler, NotificationLevel, NotificationOptions, NotifyEvery, NotifyVia, Priority, Site};
use uuid::Uuid;

use crate::config::DatabaseConfig;

pub struct Postgres {
    cfg: DatabaseConfig,
}

impl Postgres {
    pub fn new(cfg: DatabaseConfig) -> Self {
        Postgres {cfg}
    }

    pub fn add_crawler(&self, crawler: Crawler) -> Result<()> {
        tracing::info!("Save crawler to database");
        Ok(())
    }

    pub fn get_crawler(&self, crawler_id: Uuid) -> Result<Crawler> {
        let crawler = Crawler {
            id: crawler_id,
            name: "parsera_crawler".to_string(),
            user_id: Uuid::now_v7(),
            timer_rule: "2/10 * * * * *".to_string(),
            priority: Priority::Common,
            notification: NotificationOptions{
                level: NotificationLevel::JobsDone,
                via: vec![NotifyVia::Email("abobus@gmail.com".into())],
                every: None,
            },
            site: Site{
                id: Uuid::now_v7(),
                domain: "parsera".to_string(),
                start_page: "https://parsera.site".to_string(),
                page_xpaths: HashMap::from([
                    ("title".into(), "//h1".into())
                ]),
                pagination_xpaths: HashMap::from([
                    ("next_page".into(), "//a/@href".into())
                ]),
                meta: None,
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
            meta: None,
        };
        Ok(crawler)
    }
}