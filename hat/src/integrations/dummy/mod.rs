use crate::integrations::Integration;
use crate::runtime::device::{Device, DeviceType};
use crate::runtime::event::{Event, EventType};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::sleep;
use tracing::error;

lazy_static::lazy_static! {
    static ref ID_COUNTER: AtomicU64 = {
        AtomicU64::default()
    };
}

#[derive(Debug, Default)]
pub struct DummyIntegration {
    id: String,
}

impl DummyIntegration {
    pub fn new() -> Self {
        let new_id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Self {
            id: format!("DummyIntegration{new_id}"),
        }
    }
}

#[async_trait]
impl Integration for DummyIntegration {
    async fn list_devices(&self) -> Result<Vec<Device>> {
        Ok([Device {
            integration: self.get_id().to_owned(),
            id: "dummy-device-2707".into(),
            name: Some("Dummy Device".into()),
            state: Some("dummy-state".into()),
            typ: DeviceType::Dummy,
            attributes: Default::default(),
        }]
        .into())
    }

    async fn get_device(&self, id: &str) -> Result<Option<Device>> {
        if id == "dummy-device-2707" {
            Ok(Some(Device {
                integration: self.get_id().to_owned(),
                id: "dummy-device-2707".into(),
                typ: DeviceType::Dummy,
                state: Some("dummy-state".into()),
                name: Some("Dummy Device".into()),
                attributes: Default::default(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn turn_on_device(&self, _device_id: &str) -> Result<()> {
        Ok(())
    }

    async fn turn_off_device(&self, _device_id: &str) -> Result<()> {
        Ok(())
    }

    fn subscribe(&self) -> UnboundedReceiver<Event> {
        let (tx, rx) = mpsc::unbounded_channel();

        let integration_name = self.get_id().to_owned();

        tokio::spawn(async move {
            loop {
                let result = tx.send(Event {
                    typ: EventType::Dummy,
                    datetime: Utc::now().into(),
                    device: Device {
                        integration: integration_name.to_string(),
                        id: "dummy-device-2707".into(),
                        name: Some("Dummy Device".into()),
                        state: Some("dummy-state".into()),
                        typ: DeviceType::Dummy,
                        attributes: Default::default(),
                    },
                    parameters: Default::default(),
                });
                if result.is_err() {
                    error!("Failed to send event to runtime!");
                    break;
                }
                sleep(Duration::from_secs(3)).await;
            }
        });

        rx
    }

    async fn set_light_color_rgb(&self, _device_id: &str, _color: [u8; 3]) -> Result<()> {
        unimplemented!()
    }

    async fn set_light_brightness(&self, _device_id: &str, _brightness: u8) -> Result<()> {
        unimplemented!()
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}
