use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::models::notification::NotificationOptions;

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Top,
    High,
    Common,
    Low,
}

// TODO Rewrite
#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    RegisterPending,
    Registered,
    RegisterFailed,
    ScrapingPending,
    Scraped,
    ScrapingFailed,
    ExtractingPending,
    Extracted,
    ExtractingFailed,
    NotifyPending,
    Notified,
    NotifyFailed,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SchedulerCommand {
    RegisterCrawler(CommandStatus),
    ScrapePage(CommandStatus),
    ExtractPage(CommandStatus),
    StorePage(CommandStatus),
    NotifyUser(CommandStatus),
    Sleep(CommandStatus),   // TODO: think about it
}


#[derive(Debug, Serialize, Deserialize)]
pub enum CommandStatus {
    Pending,
    Done,
    Failed,
}


// TODO: refactor
#[derive(Debug, Serialize, Deserialize)]
pub struct Crawler {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub timer_rule: String,
    pub priority: Priority,
    pub notification: NotificationOptions,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub site: Site,
    pub meta: Option<String>,
}

// TODO: refactor
#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub id: Uuid,
    pub domain: String,
    pub start_page: String,
    pub page_xpaths: HashMap<String, String>,
    pub pagination_xpaths: HashMap<String, String>,
    pub meta: Option<String>,
}

// TODO: refactor
#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub id: Uuid,
    pub crawler_id: Uuid,
    pub site_id: Uuid,
    pub url: String,
    pub domain: String,
    pub is_pagination: bool,
    pub times_reparsed: u32,
    pub status: Status,
    pub priority: Priority,
    pub notification: NotificationOptions,
    pub xpaths: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub html: Option<String>,
    pub data: Option<HashMap<String, String>>,
    pub meta: Option<String>,
}