#![allow(dead_code,unused)]
use axum::{
    Json,
    extract::Path,
};
use serde::Deserialize;


pub async fn get_user(Path(user_id): Path<String>) -> &'static str {
    tracing::debug!("your user id {}", user_id);
    "your user is bip bip"
}

pub async fn get_users() -> &'static str {
    tracing::debug!("getting all users");
    "users"
}


#[derive(Deserialize, Debug)]
pub struct CreateUserPayload {
    pub first_name: String,
    pub last_name: String,
    pub country: String
}

pub async fn create_user(Json(payload): Json<CreateUserPayload>) -> &'static str {
    // tracing::debug_span!("creating user {}", payload);
    "creating user"
}
