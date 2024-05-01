// TODO: add redis client
extern crate log;
use std::collections::HashMap;

use crate::identity::Identity;

use futures::prelude::*;
use redis::{AsyncCommands, Client, aio, RedisError, JsonAsyncCommands};

pub struct Redis {
    client: Client,
    conn: aio::Connection,
}

impl Redis {
    pub async fn new(url: &str) -> Result<Self, RedisError> {
        let client = redis::Client::open(url).unwrap();
        let conn = client.get_async_connection().await?;
        let redis = Redis {
            client,
            conn,
        };
        Ok(redis)
    }

    async fn get(&mut self, key: &str) -> Option<Identity> {
        let identity_map: String = self.conn.get(key).await.unwrap();
        let identity: Identity = serde_json::from_str(&identity_map).unwrap();
        Some(identity)
    }

    async fn random_key(&mut self) -> Option<String> {
        let key: String = match redis::cmd("RANDOMKEY").query_async(&mut self.conn).await {
            Ok(key) => key,
            Err(_) => {
                log::warn!("No keys in redis");
                return None;
            },
        };
        Some(key)
    }

    pub async fn get_identity(&mut self) -> Option<Identity> {
        let key = match self.random_key().await {
            Some(key) => key,
            None => return None,
        };
        let identity: Identity = match self.get(&key).await {
            Some(identity) => identity,
            None => return None,
        };
        Some(identity)
    }
}