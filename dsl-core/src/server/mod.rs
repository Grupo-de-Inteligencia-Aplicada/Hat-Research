use std::sync::Arc;

use axum::{routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::runtime::HatRuntime;

mod transpiler;

#[derive(Clone)]
struct AppState {
    pub runtime: Arc<HatRuntime>,
}

pub fn make_router(runtime: Arc<HatRuntime>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any);
    Router::new()
        .route(
            "/transpile/into_xml",
            post(transpiler::transpile_hat_to_workspace),
        )
        .route(
            "/transpile/into_hat",
            post(transpiler::transpile_workspace_to_hat),
        )
        .layer(cors)
        .with_state(AppState { runtime })
}
