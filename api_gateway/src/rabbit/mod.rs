#![allow(dead_code,unused)]
mod broker;

use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::{request::Parts, StatusCode}};
use lapin::Channel;

pub use broker::*;


struct RabbitExtractor(Channel);

#[async_trait]
impl <S> FromRequestParts<S> for RabbitExtractor
where
    RabbitMQ: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let rabbit = RabbitMQ::from_ref(state);
        let channel = rabbit.get_channel().await.map_err(internal_error)?;
        Ok(Self(channel))
    }
}


pub fn internal_error<E: std::error::Error>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}