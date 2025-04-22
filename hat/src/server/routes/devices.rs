use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    runtime::device::Device,
    server::{
        error::{ApiResult, RaiseInternalError},
        AppState,
    },
};

#[axum::debug_handler]
pub async fn get_devices(State(state): State<AppState>) -> ApiResult<Json<Vec<Device>>> {
    let integrations = state.runtime.get_integrations().await;

    let futures = integrations
        .iter()
        .map(|i| {
            let i = Arc::clone(i);
            tokio::spawn(async move { i.list_devices().await })
        })
        .collect::<Vec<_>>();

    let mut devices = Vec::new();

    for future in futures {
        let mut result = future
            .await
            .raise_internal_error(Some("failed to list devices"))?
            .raise_internal_error(Some("failed to list devices"))?;
        devices.append(&mut result);
    }

    Ok(Json(devices))
}

#[derive(Deserialize)]
pub struct GetDeviceQuery {
    id: String,
}

pub async fn get_device(
    Query(query): Query<GetDeviceQuery>,
    State(state): State<AppState>,
) -> ApiResult<Json<Option<Device>>> {
    let dev = state
        .runtime
        .get_device(&query.id)
        .await
        .raise_internal_error(None)?;

    Ok(Json(dev))
}
