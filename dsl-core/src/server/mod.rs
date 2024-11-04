use std::sync::Arc;

use axum::Router;

use crate::runtime::HatRuntime;

pub fn make_router(runtime: Arc<HatRuntime>) -> Router {
    let router = Router::new();

    router
}
