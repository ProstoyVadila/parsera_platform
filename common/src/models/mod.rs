use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod notification;
mod crawler;

pub use notification::*;
pub use crawler::*;

// TODO: remove this
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum EventStatus {
    Pending,
    RegisterError,
    ScrapperProcessing,
    ScrapperDone,
    ScrapperError,
    ParserProcessing,
    ParserError,
    ParserDone,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Xpath {
//     pub field_name: String,
//     pub value: String,
//     pub data: String,
// }
    
// TODO: remove this
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddSiteEvent {
    pub id: usize,
    pub url: String,
    pub user_id: usize,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub xpaths: HashMap<String, String>,
    pub status: EventStatus,
}
