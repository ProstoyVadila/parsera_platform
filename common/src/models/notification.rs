use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationLevel {
    JobsDone,       // When job's done, failed, stats
    JobsFailed,     // When failed and global stats
    Statistics,     // stats only
    DoNotDisturb,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NotifyEvery {
    Day,
    Week,
    Month,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum NotifyVia {
    Email(String),
    Telegram(String),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationOptions {
    pub level: NotificationLevel,
    pub via: Vec<NotifyVia>,
    pub every: Option<NotifyEvery>,
}
