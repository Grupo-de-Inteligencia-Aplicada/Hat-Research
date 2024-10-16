use super::HAWebSocket;
use crate::integrations::home_assistant::events::{Event as HassEvent, EventData};
use crate::runtime::device::DeviceType;
use crate::runtime::event::{Event as RuntimeEvent, EventType};
use crate::{integrations::Integration, runtime::device::Device};
use anyhow::{bail, ensure, Result};
use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::{debug, error};
use url::Url;

lazy_static::lazy_static! {
    static ref ID_COUNTER: AtomicU64 = {
        AtomicU64::default()
    };
}

pub struct HassIntegration {
    http_client: reqwest::Client,
    url: Url,
    ws: Arc<HAWebSocket>,
    id: String,
}

impl HassIntegration {
    pub async fn new(hass_url: &str, access_token: &str) -> Result<Self> {
        let url = Url::parse(hass_url)?;
        ensure!(
            url.scheme() == "http" || url.scheme() == "https",
            "unknown url scheme"
        );
        let ws_url = {
            let mut ws_url = url.clone();
            if let Err(e) = ws_url.set_scheme(match ws_url.scheme() {
                "http" => "ws",
                "https" => "wss",
                _ => unreachable!(),
            }) {
                bail!("failed to update hass url: {e:?}");
            }
            ws_url.set_path("/api/websocket");
            ws_url
        };
        let ws = HAWebSocket::connect(ws_url.to_string().as_ref(), access_token).await?;
        let new_id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let http_client = reqwest::Client::builder()
            .default_headers(
                HeaderMap::try_from(&HashMap::from([(
                    "Authorization".to_owned(),
                    format!("Bearer {access_token}"),
                )]))
                .unwrap(),
            )
            .build()?;
        Ok(Self {
            http_client,
            url,
            ws: Arc::new(ws),
            id: format!("HassIntegration{new_id}"),
        })
    }
    fn get_endpoint_from_api_route(&self, route: &str) -> Url {
        let mut url = self.url.clone();
        url.set_path(route);
        url
    }
    fn get_device_type_from_entity_id(entity_id: &str, device_class: Option<&str>) -> DeviceType {
        let typ = entity_id.split_once(".");
        if let Some((typ, _)) = typ {
            match typ {
                "light" => DeviceType::Light,
                "sensor" => DeviceType::Sensor,
                "binary_sensor" => {
                    match device_class {
                        Some("door") => DeviceType::DoorSensor,
                        Some(_) => DeviceType::Unknown,
                        _ => DeviceType::Unknown,
                    }
                },
                _ => DeviceType::Unknown,
            }
        } else {
            DeviceType::Unknown
        }
    }
}

#[derive(Deserialize)]
struct HassEntityState {
    attributes: HashMap<String, serde_json::Value>,
    entity_id: String,
    state: serde_json::Value,
}

#[async_trait]
impl Integration for HassIntegration {
    async fn list_devices(&self) -> Result<Vec<Device>> {
        let res = self
            .http_client
            .get(self.get_endpoint_from_api_route("/api/states"))
            .send()
            .await?;

        ensure!(
            res.status() == StatusCode::OK,
            "failed to list devices on hass: {}, {}",
            res.status(),
            res.text().await?,
        );

        let devices = res
            .json::<Vec<HassEntityState>>()
            .await?
            .into_iter()
            .map(|entity| {
                Device {
                    integration: self.get_id().to_owned(),
                    typ: Self::get_device_type_from_entity_id(
                        &entity.entity_id,
                        entity.attributes.get("device_class").and_then(|c| c.as_str()),
                    ),
                    id: entity.entity_id,
                    name: None, // TODO: get this property, if possible
                    state: Some(
                        entity
                            .state
                            .as_str()
                            .expect("states that are not a string are not yet implemented")
                            .to_owned(),
                    ),
                }
            })
            .collect();

        Ok(devices)
    }

    async fn get_device(&self, id: &str) -> Result<Option<Device>> {
        let res = self
            .http_client
            .get(self.get_endpoint_from_api_route(&format!("/api/states/{id}")))
            .send()
            .await?;

        if res.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        ensure!(
            res.status() == StatusCode::OK,
            "failed to list devices on hass: {}, {}",
            res.status(),
            res.text().await?,
        );

        let res = res.json::<HassEntityState>().await?;

        let device = Device {
            integration: self.get_id().to_owned(),
            typ: Self::get_device_type_from_entity_id(
                &res.entity_id,
                res.attributes.get("device_class").and_then(|c| c.as_str())
            ),
            id: res.entity_id,
            name: None, // TODO: get this property, if possible
            state: Some(
                res.state
                    .as_str()
                    .expect("states that are not a string are not yet implemented")
                    .to_owned(),
            ),
        };

        Ok(Some(device))
    }

    fn subscribe(&self) -> UnboundedReceiver<RuntimeEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        let api = Arc::clone(&self.ws);

        let integration_name = self.get_id().to_owned();

        tokio::spawn(async move {
            let mut events = match api.subscribe_events(None).await {
                Ok(events) => events,
                Err(e) => {
                    error!("Failed to subscribe to home assistant events: {e:#?}");
                    return;
                }
            };

            loop {
                let hass_event = match events.next().await {
                    Ok(event) => event,
                    Err(e) => {
                        error!("Failed to read event from home assistant: {e:#?}");
                        break;
                    }
                };

                let runtime_event = parse_event(&integration_name, &hass_event);

                if let Some(runtime_event) = runtime_event {
                    tx.send(runtime_event).unwrap();
                } else {
                    debug!(
                        "Event not recognized: {}",
                        serde_json::to_string_pretty(&hass_event).unwrap_or_default()
                    );
                }
            }
        });

        rx
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

fn parse_event(integration_name: &str, hass_event: &HassEvent) -> Option<RuntimeEvent> {
    let time = DateTime::parse_from_rfc3339(&hass_event.time_fired).ok()?;
    let time = Utc.from_utc_datetime(&time.naive_utc());
    match &hass_event.data {
        EventData::StateChanged {
            entity_id,
            old_state: old_state_data,
            new_state: new_state_data,
        } => {
            let entity_type = entity_id.split(".").next()?;
            let new_state = new_state_data.get("state")?;
            let old_state = old_state_data.get("state")?;
            let attribs = new_state_data.get("attributes")?;
            let name = attribs
                .get("friendly_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_owned());

            match entity_type {
                "binary_sensor" => {
                    let attribs = new_state_data.get("attributes")?;
                    let device_class = attribs.get("device_class").and_then(|v| v.as_str());
                    if device_class == Some("door") {
                        let device = Device {
                            integration: integration_name.to_owned(),
                            id: entity_id.to_owned(),
                            name,
                            state: Some(new_state.to_string()),
                            typ: DeviceType::DoorSensor,
                        };
                        if old_state == "off" && new_state == "on" {
                            return Some(RuntimeEvent {
                                typ: EventType::DoorOpenEvent,
                                datetime: time,
                                device,
                                parameters: Default::default(),
                            });
                        }
                        if old_state == "on" && new_state == "off" {
                            return Some(RuntimeEvent {
                                typ: EventType::DoorCloseEvent,
                                datetime: time,
                                device,
                                parameters: Default::default(),
                            });
                        }
                    }
                }
                "light" => {
                    let device = Device {
                        integration: integration_name.to_owned(),
                        id: entity_id.to_owned(),
                        name,
                        state: Some(new_state.to_string()),
                        typ: DeviceType::Light,
                    };
                    if old_state == "off" && new_state == "on" {
                        return Some(RuntimeEvent {
                            typ: EventType::LightOnEvent,
                            datetime: time,
                            device,
                            parameters: Default::default(),
                        });
                    }
                    if old_state == "on" && new_state == "off" {
                        return Some(RuntimeEvent {
                            typ: EventType::LightOffEvent,
                            datetime: time,
                            device,
                            parameters: Default::default(),
                        });
                    }
                }
                "sensor" => {
                    let mut parameters = HashMap::new();
                    let value = new_state.as_str();
                    if let Some(value) = value {
                        parameters.insert("value".into(), value.to_owned());
                    }
                    return Some(RuntimeEvent {
                        typ: EventType::SensorValueChangeEvent,
                        datetime: time,
                        device: Device {
                            integration: integration_name.to_owned(),
                            id: entity_id.to_owned(),
                            name,
                            state: Some(new_state.to_string()),
                            typ: DeviceType::Sensor,
                        },
                        parameters,
                    });
                }
                _ => {}
            }
        }
        EventData::Unknown { .. } => {}
    };
    None
}
