use std::sync::Arc;

use axum::Router;

use crate::runtime::HatRuntime;

struct AppState {
    runtime: Arc<HatRuntime>,
}

pub fn make_router(runtime: Arc<HatRuntime>) -> Router {
    let router = Router::new()
        .with_state(AppState {
            runtime
        });

    router
}
