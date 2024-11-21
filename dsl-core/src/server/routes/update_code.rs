use axum::extract::State;

use crate::server::{error::{ApiError, ApiResult}, AppState};

pub async fn update_code(State(state): State<AppState>, src: String) -> ApiResult<()> {
    if let Err(e) = state.runtime.replace_source("web-source.hat".into(), &src) {
        Err(ApiError::bad_request(format!("failed to parse source: {e:#?}")))
    } else {
        Ok(())
    }
}
