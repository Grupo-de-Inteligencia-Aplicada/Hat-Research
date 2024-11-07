use std::sync::Arc;

use axum::{routing::post, Router};

use crate::runtime::HatRuntime;

mod transpiler;

#[derive(Clone)]
struct AppState {
    pub runtime: Arc<HatRuntime>,
}

pub fn make_router(runtime: Arc<HatRuntime>) -> Router {
    Router::new()
        .route(
            "/transpile/into_xml",
            post(transpiler::transpile_hat_to_workspace),
        )
        .route(
            "/transpile/into_hat",
            post(transpiler::transpile_workspace_to_hat),
        )
        .with_state(AppState { runtime })
}
