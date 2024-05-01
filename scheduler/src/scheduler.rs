use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;
use tokio_cron_scheduler::JobScheduler;


pub struct Scheduler {
    pub cfg: String,
    pub broker: String,
    pub database: String,
    pub orchestrator: String,
    pub api: String,
    pub innner: Arc<RwLock<JobScheduler>>,
}

impl Scheduler {
    pub async fn new() -> Result<Self> {
        todo!()
    }

    async fn init_broker() -> Result<()> {
        todo!()
    }

    async fn init_database() -> Result<()> {
        todo!()
    }

    async fn init_http_server() -> Result<()> {
        todo!()
    }

    async fn init_grpc_server() -> Result<()> {
        // TODO: migrate from http to grpc
        todo!()
    }

    pub async fn run() -> Result<()> {
        todo!()
    }
}