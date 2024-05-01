#![allow(unused)]
use std::string::ToString;

use strum_macros::Display;
use serde::{Serialize, Deserialize};

use super::*;


#[derive(Debug, Display, Serialize, Deserialize, Clone)]
pub enum EventCommand {
    #[serde(alias = "register_crawler")]
    RegisterCrawler(EventCommandStatus),
    #[serde(alias = "scrape_page")]
    ScrapePage(EventCommandStatus),
    #[serde(alias = "extract_page")]
    ExtractPage(EventCommandStatus),
    #[serde(alias = "store_page")]
    StorePage(EventCommandStatus),
    #[serde(alias = "notify_user")]
    NotifyUser(EventCommandStatus),
    #[serde(alias = "sleep")]
    Sleep(EventCommandStatus),   // TODO: think about it
}

#[derive(Debug, Display, Serialize, Deserialize, Clone)]
pub enum EventCommandStatus {
    #[serde(alias = "pending")]
    Pending,
    #[serde(alias = "done")]
    Done,
    #[serde(alias = "failed")]
    Failed,
}

#[derive(Debug, Display, Serialize, Deserialize)]
pub enum EventProtocolData {
    #[serde(alias = "external")]
    External(Crawler),
    #[serde(alias = "internal")]
    Internal(Page),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventProtocol {
    pub command: EventCommand,
    pub data: EventProtocolData,
}
