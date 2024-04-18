use axum::{Router, routing::{get, post}};

use crate::{app::SharedState, api};

pub fn api() -> Router<SharedState> {
    Router::new()
        .route("/healthcheck", get(api::healthcheck))
        .route("/user/:id", get(api::get_user))
        .route("/user", post(api::create_user))
        .route("/users", get(api::get_users))
        .route("/add_site", get(api::add_site))
}