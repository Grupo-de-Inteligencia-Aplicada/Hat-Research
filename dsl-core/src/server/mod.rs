use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use http::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};

use crate::runtime::HatRuntime;

mod transpiler;
mod routes;
mod error;

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
        .route(
            "/transpile/into_xml",
            post(transpiler::transpile_hat_to_workspace),
        )
        .route(
            "/transpile/into_hat",
            post(transpiler::transpile_workspace_to_hat),
        )
        .route(
            "/devices",
            get(routes::devices::get_devices),
        )
        .layer(cors)
        .with_state(AppState { runtime })
}
