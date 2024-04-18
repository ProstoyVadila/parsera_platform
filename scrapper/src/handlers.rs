use std::{
    error::Error,
    collections::HashMap,
};

use crate::requests::Requests;
use crate::models::EventStatus;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageIn {
    pub event_id: String,
    pub user_id: String,
    pub url: String,
    pub xpaths: HashMap<String, String>,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub status: EventStatus,
}

#[derive(Serialize, Deserialize)]
pub struct MessageOut {
    pub html: String,
    pub event_id: String,
    pub url: String,
    pub user_id: String,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub xpaths: HashMap<String, String>,
    pub statuts: EventStatus,
}

pub async fn handle_srcap_event(data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let msg_in = serde_json::from_str::<MessageIn>(data).expect("deserialize error in handle_parse_event");
    
    let requests = Requests::new();
    let html = requests.get(&msg_in.url).await?;

    let msg_out = MessageOut {
        html: html,
        event_id: msg_in.event_id,
        user_id: msg_in.user_id,
        url: msg_in.url,
        xpaths: msg_in.xpaths,
        is_pagination: msg_in.is_pagination,
        refresh_interval: msg_in.refresh_interval,
        statuts: msg_in.status,
    };
    let out = serde_json::to_vec(&msg_out).expect("serialize error in handle_parse_event");
    Ok(out)
}