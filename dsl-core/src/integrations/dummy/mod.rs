use crate::integrations::Integration;
use crate::runtime::device::{Device, DeviceType};
use crate::runtime::event::{Event, EventType};
use async_trait::async_trait;
use chrono::Utc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::sleep;
use tracing::error;

#[derive(Debug, Default)]
pub struct DummyIntegration;

#[async_trait]
impl Integration for DummyIntegration {
    async fn list_devices(&self) -> Vec<Device> {
        [Device {
            integration: self.name().to_owned(),
            id: "dummy-device-2707".into(),
            name: Some("Dummy Device".into()),
            typ: DeviceType::Dummy,
        }]
        .into()
    }

    fn subscribe(&self) -> UnboundedReceiver<Event> {
        let (tx, rx) = mpsc::unbounded_channel();

        let integration_name = self.name();

        tokio::spawn(async move {
            loop {
                let result = tx.send(Event {
                    typ: EventType::Dummy,
                    datetime: Utc::now(),
                    device: Device {
                        integration: integration_name.to_string(),
                        id: "dummy-device-2707".into(),
                        name: Some("Dummy Device".into()),
                        typ: DeviceType::Dummy,
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

    fn name(&self) -> &'static str {
        "DummyIntegration"
    }
}
