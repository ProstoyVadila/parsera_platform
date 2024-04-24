// #![allow(dead_code,unused)]
use actix_web::{get, post, web, App, HttpServer};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio_cron_scheduler::Job;
use tracing_actix_web::{RequestId, TracingLogger};

use crate::config::Config;
use crate::SharedSheduler;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RoutineIn {
    name: String,
    rule: String,
    task: String,
}

#[derive(Debug, Serialize)]
struct RoutineOut {
    info: RoutineIn,
    routine_id: String,
}

#[post("/routine")]
async fn add_routine(
    routine: web::Json<RoutineIn>,
    sched: web::Data<SharedSheduler>,
    req_id: RequestId,
) -> web::Json<RoutineOut> {
    tracing::debug!("Adding a new routine for {}: {:?}", req_id, routine);

    let rtn = routine.clone();
    let job = Job::new(rtn.rule.as_str(), move |_uuid, _lock| {
        tracing::warn!("{}: {}", rtn.name, rtn.task);
    })
    .unwrap_or_else(|_| panic!("cannot register a new job for request {}", req_id));

    let job_id = sched
        .lock()
        .await
        .add(job)
        .await
        .expect("cannot register a a new job");

    web::Json(RoutineOut {
        info: routine.0,
        routine_id: job_id.to_string(),
    })
}

#[get("/healthcheck")]
async fn get_healthcheck() -> &'static str {
    tracing::debug!("got healthcheck");
    "ok"
}

pub async fn run_server(cfg: Config, sched: SharedSheduler) -> Result<()> {
    tracing::info!("Starting web server on {}", cfg.get_socket_addr());
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(sched.clone()))
            .service(add_routine)
            .service(get_healthcheck)
    })
    .bind(cfg.get_socket_addr())?
    .run()
    .await?;
    Ok(())
}
