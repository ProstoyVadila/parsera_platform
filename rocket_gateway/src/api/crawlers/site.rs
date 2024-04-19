#![allow(dead_code,unused)]
use std::collections::HashMap;

use rocket::serde::{Serialize, Deserialize, json::Json};

use common::models::{AddSiteEvent, EventStatus};


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AddSiteIn {
    pub url: String,
    pub user_id: usize,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub xpaths: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AddSitedOut {
    pub params: AddSiteIn,
    pub event_id: usize,
    pub status: EventStatus,
}

#[post("/add_site", format = "json", data = "<payload>")]
pub async fn add_site(payload: Json<AddSiteIn>) -> Json<AddSitedOut> {
    let Json(payload) = payload;
    Json(AddSitedOut { params: payload, event_id: 1, status: EventStatus::Pending })
}
