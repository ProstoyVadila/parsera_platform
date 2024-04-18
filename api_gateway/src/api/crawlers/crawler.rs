use std::collections::HashMap;
use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use axum::debug_handler;
use serde::{Deserialize, Serialize};

use common::models::{EventStatus, AddSiteEvent};

use crate::rabbit::RabbitMQ;
use crate::app::SharedState;


#[derive(Deserialize)]
pub struct CrawlerPayloadIn {
    pub user_id: String,
    pub crawler_id: String,
}

#[derive(Serialize)]
pub struct CrawlerPayloadOut {
    pub user_id: String,
    pub crawler_id: String,
    pub options: String,
}

pub async fn get_crawlers(Json(payload): Json<CrawlerPayloadIn>) -> Json<CrawlerPayloadOut> {
    let out = CrawlerPayloadOut {
        user_id: payload.user_id,
        crawler_id: payload.crawler_id,
        options: "damn.".to_string(),
    };
    Json(out)
}


pub struct CreateCrawlerPayloadIn {

}

pub async fn create_crawler(
    Json(payload): Json<CrawlerPayloadIn>,
    State(state): State<SharedState>,
) -> Json<CrawlerPayloadOut> {
    
    let out = CrawlerPayloadOut {
        user_id: payload.user_id,
        crawler_id: payload.crawler_id,
        options: "registered".to_string(),
    };

    let task = serde_json::to_string(&out).expect("task serialized");
    let channel = state.rabbit.get_channel().await.expect("hehe");
    let confirm = state.rabbit.publish(channel, task.as_bytes()).await;
    // let rabbit = &state.

    Json(out)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddSitePayloadIn {
    pub url: String,
    pub user_id: usize,
    pub is_pagination: bool,
    pub refresh_interval: usize,
    pub xpaths: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct AddSitePayloadOut {
    pub params: AddSitePayloadIn,
    pub event_id: usize,
    pub status: EventStatus,
    
}

#[debug_handler]
pub async fn add_site(
    State(state): State<SharedState>,
    Json(payload): Json<AddSitePayloadIn>, 
) -> Json<AddSitePayloadOut> {

    let event = AddSiteEvent {
        id: 1,
        url: payload.url.clone(),
        user_id: payload.user_id,
        is_pagination: payload.is_pagination,
        refresh_interval: payload.refresh_interval,
        xpaths: payload.xpaths.clone(),
        status: EventStatus::Pending,
    };
    let event_json = match serde_json::to_string(&event) {
        Ok(json) => json,
        Err(err) => {
            tracing::error!("cannot serialize add site event: {}", err);
            return Json(AddSitePayloadOut{
                params: payload.clone(),
                event_id: event.id,
                status: EventStatus::RegisterError,
            });
        }
    };

    let channel = match state.rabbit.get_channel().await {
        Ok(ch) => ch,
        Err(err) => {
            tracing::error!("cannot get channel: {}", err);
            return Json(AddSitePayloadOut{
                params: payload.clone(),
                event_id: event.id,
                status: EventStatus::RegisterError,
            });
        }
    };
    let _ = match state.rabbit.publish(channel, event_json.as_bytes()).await {
        Ok(ok) => ok,
        Err(err) => {
            tracing::error!("cannot publish message: {}", err);
            return Json(AddSitePayloadOut{
                params: payload.clone(),
                event_id: event.id,
                status: EventStatus::RegisterError,
            }); 
        }
    };

    let out = AddSitePayloadOut {
        event_id: event.id,
        params: payload,
        status: event.status,
    };
    Json(out)
}