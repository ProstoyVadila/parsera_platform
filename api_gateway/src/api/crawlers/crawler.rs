#![allow(dead_code,unused)]
use std::collections::HashMap;

use rocket::serde::{Serialize, Deserialize, json::Json};

use common::models::{AddSiteEvent, EventStatus};

// GET
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetCrawlerIn {
    pub url: String,
    pub user_id: usize,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub xpaths: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetCrawlerOut {
    pub params: GetCrawlerIn,
    pub event_id: usize,
    pub status: EventStatus,
}

#[get("/crawler", format = "json", data = "<payload>")]
pub async fn get_crawler(payload: Json<GetCrawlerIn>) -> Json<GetCrawlerOut> {
    todo!("impl")
}


// CREATE
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AddCrawlerIn {
    pub url: String,
    pub user_id: usize,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub xpaths: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AddCrawlerOut {
    pub params: AddCrawlerIn,
    pub event_id: usize,
    pub status: EventStatus,
}

#[post("/crawler", format = "json", data = "<payload>")]
pub async fn add_crawler(payload: Json<AddCrawlerIn>) -> Json<AddCrawlerOut> {
    todo!("impl")
}


// DELETE
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteCrawlerIn {
    pub url: String,
    pub user_id: usize,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub xpaths: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteCrawlerOut {
    pub params: DeleteCrawlerIn,
    pub event_id: usize,
    pub status: EventStatus,
}

#[delete("/crawler", format = "json", data = "<payload>")]
pub async fn delete_crawler(payload: Json<DeleteCrawlerIn>) -> Json<DeleteCrawlerOut> {
    todo!("impl")
}


// UPDATE
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateCrawlerIn {
    pub url: String,
    pub user_id: usize,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub xpaths: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateCrawlerOut {
    pub params: UpdateCrawlerIn,
    pub event_id: usize,
    pub status: EventStatus,
}

#[put("/crawler", format = "json", data = "<payload>")]
pub async fn update_crawler(payload: Json<UpdateCrawlerIn>) -> Json<UpdateCrawlerOut> {
    todo!("impl")
}

// GET ALL
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetCrawlersIn {
    pub url: String,
    pub user_id: usize,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub xpaths: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetCrawlersOut {
    pub params: GetCrawlerIn,
    pub event_id: usize,
    pub status: EventStatus,
}

#[get("/crawlers", format = "json", data = "<payload>")]
pub async fn get_crawlers(payload: Json<GetCrawlersIn>) -> Json<GetCrawlersOut> {
    todo!("impl")
}