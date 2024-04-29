use serde::{Serialize, Deserialize};

use super::{Crawler, Page};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SchedulerCommand {
    #[serde(alias = "register_crawler")]
    RegisterCrawler(CommandStatus),
    #[serde(alias = "scrape_page")]
    ScrapePage(CommandStatus),
    #[serde(alias = "extract_page")]
    ExtractPage(CommandStatus),
    #[serde(alias = "store_page")]
    StorePage(CommandStatus),
    #[serde(alias = "notify_user")]
    NotifyUser(CommandStatus),
    #[serde(alias = "sleep")]
    Sleep(CommandStatus),   // TODO: think about it
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommandStatus {
    #[serde(alias = "pending")]
    Pending,
    #[serde(alias = "done")]
    Done,
    #[serde(alias = "failed")]
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageToScheduler {
    pub action: SchedulerCommand,
    pub crawler: Option<Crawler>,
    pub page: Option<Page>,
}