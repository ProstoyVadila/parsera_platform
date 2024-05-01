#![allow(unused)]
use std::string::ToString;

use serde::{Serialize, Deserialize};
use strum_macros::Display;


#[derive(Debug, Display, Serialize, Deserialize, Clone)]
pub enum NotificationLevel {
    JobsDone,       // When job's done, failed, stats
    JobsFailed,     // When failed and global stats
    Statistics,     // stats only
    DoNotDisturb,
}

#[derive(Debug, Display, Serialize, Deserialize, Clone)]
pub enum NotifyEvery {
    Day,
    Week,
    Month,
}


#[derive(Debug, Display, Serialize, Deserialize, Clone)]
pub enum NotifyVia {
    Email(String),  // email
    Telegram(String),   //telegram user id ?
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationOptions {
    pub level: NotificationLevel,
    pub via: Vec<NotifyVia>,
    pub every: Option<NotifyEvery>,
}
