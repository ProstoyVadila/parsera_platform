use serde::{Serialize, Deserialize};

use super::{Crawler, Page};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SchedulerCommand {
    RegisterCrawler(CommandStatus),
    ScrapePage(CommandStatus),
    ExtractPage(CommandStatus),
    StorePage(CommandStatus),
    NotifyUser(CommandStatus),
    Sleep(CommandStatus),   // TODO: think about it
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommandStatus {
    Pending,
    Done,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageToScheduler {
    pub action: SchedulerCommand,
    pub crawler: Option<Crawler>,
    pub page: Option<Page>,
}