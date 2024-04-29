use serde::{Serialize, Deserialize};

use super::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct MockCrawler {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockPage {
    pub page_id: usize,
    pub url: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum EventProtocolData {
    #[serde(alias = "external")]
    External(MockCrawler),
    #[serde(alias = "internal")]
    Internal(MockPage),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventProtocol {
    pub command: SchedulerCommand,
    pub data: EventProtocolData,
}