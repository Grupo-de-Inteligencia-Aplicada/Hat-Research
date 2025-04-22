use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use http::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};

use crate::runtime::HatRuntime;

mod error;
mod routes;

#[derive(Clone)]
struct AppState {
    pub runtime: Arc<HatRuntime>,
}

pub fn make_router(runtime: Arc<HatRuntime>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/devices", get(routes::devices::get_devices))
        .route("/device", get(routes::devices::get_device))
        .route("/possible_events", get(routes::events::get_possible_events))
        .route("/update_code", post(routes::update_code::update_code))
        .layer(cors)
        .with_state(AppState { runtime })
}
