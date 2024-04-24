
#![allow(dead_code,unused)]
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::HttpMessage;
use anyhow::Result;
use actix_web::{http::header::ContentType, get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::{Compat, Logger};
use tracing_actix_web::{TracingLogger, RequestId};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use serde::{Deserialize, Serialize};

mod config;
mod jobs;


pub type SharedSheduler = Arc<Mutex<JobScheduler>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Routine {
    name: String,
    rule: String,
    task: String,
}

#[post("/routine")]
async fn add_routine(routine: web::Json<Routine>, sched: web::Data<SharedSheduler>, req_id: RequestId) -> web::Json<Routine> {
    tracing::debug!("Adding a new routine for {}: {:?}", req_id, routine);

    let rtn = routine.clone();
    let job = Job::new(rtn.rule.as_str(), move |_uuid, _lock| {
        tracing::warn!("{}: {}", rtn.name, rtn.task);
    }).expect(format!("cannot register a new job for request {}", req_id).as_str());

    sched.lock().unwrap().add(job).await;

    routine
}

#[get("/healthcheck")]
async fn get_healthcheck() -> &'static str {
    tracing::debug!("got healthcheck");
    "ok"
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::new();
    cfg.init_tracing();

    let mut sched = Arc::new(Mutex::new(JobScheduler::new().await?));
    
    jobs::register_jobs(&mut sched.clone()).await?;
    
    tracing::info!("Starting scheduler...");
    sched.lock().expect("cannot start scheduler").start().await?;

    tracing::info!("Starting web server...");
    HttpServer::new(move || {
        App::new()
        .wrap(TracingLogger::default())
        .app_data(web::Data::new(sched.clone()))
        .service(add_routine)
        .service(get_healthcheck)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await;
    Ok(())
}
