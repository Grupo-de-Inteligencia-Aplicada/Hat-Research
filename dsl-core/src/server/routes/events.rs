use std::{collections::HashSet, sync::Arc};

use axum::{extract::State, Json};
use serde::Serialize;

use crate::{
    runtime::event::EventType,
    server::{
        error::{ApiResult, RaiseInternalError},
        AppState,
    },
};

#[derive(Serialize)]
pub struct EventInfo<'a> {
    event: EventType,
    description: &'a str,
}

#[axum::debug_handler]
pub async fn get_possible_events(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<EventInfo<'static>>>> {
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

    let types = devices
        .into_iter()
        .map(|d| d.typ)
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|device_type| EventType::get_events_related_to(device_type))
        .flat_map(|slice| slice.iter().cloned())
        .map(|event| EventInfo {
            event,
            description: event.get_description(),
        })
        .collect::<Vec<_>>();

    Ok(Json(types))
}
