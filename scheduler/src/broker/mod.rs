use anyhow::Result;

pub use broker::*;

use crate::utils::ParseraService;

mod broker;


// TODO: finish this
pub trait Broker {
    async fn publish(payload: &[u8], to: ParseraService) -> Result<()>;
    async fn consume() -> Result<()>;
}