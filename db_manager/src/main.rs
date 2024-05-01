use anyhow::Result;
use uuid::Uuid;
use scylla::{Session, SessionBuilder};
use scylla::FromRow;
use scylla::ValueList;

mod duration;
mod repo;
// use chrono::Duration;

#[derive(Debug, FromRow, ValueList)]
pub struct TemperatureMeasurement {
    pub device: Uuid,
    // pub time: duration::Duration,
    pub temperature: i16,
}

async fn create_session(url: &str) -> Result<Session> {
    SessionBuilder::new()
        .known_node(url)
        .build()
        .await
        .map_err(From::from)

}

#[tokio::main]
async fn main() -> Result<()> {
    
    let session = create_session("127.0.0.1:7199").await?;

    Ok(())
}
