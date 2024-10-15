use super::HAWebSocket;
use crate::integrations::home_assistant::events::{Event as HassEvent, EventData};
use crate::runtime::device::DeviceType;
use crate::runtime::event::{Event as RuntimeEvent, EventType};
use crate::{integrations::Integration, runtime::device::Device};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::{debug, error};

lazy_static::lazy_static! {
    static ref ID_COUNTER: AtomicU64 = {
        AtomicU64::default()
    };
}

pub struct HassIntegration {
    ws: Arc<HAWebSocket>,
    id: String,
}

impl HassIntegration {
    pub async fn new(hass_url: &str, access_token: &str) -> Result<Self> {
        let ws = HAWebSocket::connect(hass_url, access_token).await?;
        let new_id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Ok(Self {
            ws: Arc::new(ws),
            id: format!("HassIntegration{new_id}"),
        })
    }
}

#[async_trait]
impl Integration for HassIntegration {
    async fn list_devices(&self) -> Vec<Device> {
        todo!()
    }

    async fn get_device(&self, id: &str) -> Option<Device> {
        todo!()
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
