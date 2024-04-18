use crate::{
    config::Config,
    redis::Redis,
};

use std::collections::VecDeque;

use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proxy {
    pub ip: String,
    pub port: u16,
    country: String,
    anonymity: String,
    google: bool,
    https: bool,
    last_checked: String,
    proxy_type: String,
    response_time: String,
    uptime: String,
    verified: bool,
    source: String,
}

impl Proxy {
    pub fn new() -> Self {
        Proxy {
            ip: String::new(),
            port: 0,
            country: String::new(),
            anonymity: String::new(),
            google: false,
            https: false,
            last_checked: String::new(),
            proxy_type: String::new(),
            response_time: String::new(),
            uptime: String::new(),
            verified: false,
            source: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identity {
    pub user_agent: String,
    pub accept: String,
    pub accept_language: String,
    pub accept_encoding: String,
    pub referer: String,
    pub cookie: String,
    pub dnt: String,
    pub upgrade_insecure_requests: String,
    pub cache_control: String,
}


pub struct IdentityManager {
    pub pool: VecDeque<Identity>,
    pub redis: Redis,
    pub rotation_rate: f32, // 0 - no rotation (only pool), 1 - only redis
}


impl IdentityManager {
    pub async fn new(pool_size: usize, redis_url: &str, rotation_rate: f32) -> Self {
        let redis = Redis::new(redis_url).await.unwrap();
        IdentityManager {
            pool: VecDeque::with_capacity(pool_size),
            redis,
            rotation_rate,
        }
    }

    pub fn add(&mut self, proxy: Identity) {
        self.pool.push_back(proxy);
    }

    pub async fn get(&mut self) -> Option<Identity> {
        if self.pool.is_empty() {
            return self.get_from_redis().await;
        }
        if self.pool.len() == self.pool.capacity() {
            return self.pool.pop_back();
        }
        match random() {
            true => self.get_from_redis().await,
            false => self.pool.pop_back(),
        }
    }

    async fn get_from_redis(&mut self) -> Option<Identity> {
        let identity = match self.redis.get_identity().await {
            Some(identity) => identity,
            None => return None,
        };
        self.add(identity.clone());
        Some(identity)
    }
}
