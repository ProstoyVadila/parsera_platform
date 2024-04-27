use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationLevel {
    JobsDone,       // When job's done or failed
    JobsFailed,     // When failed only
    DoNotDisturb,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum NotifyVia {
    Email(String),
    Telegram(String),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationOptions {
    pub via: Vec<NotifyVia>,
    pub level: NotificationLevel,
}
