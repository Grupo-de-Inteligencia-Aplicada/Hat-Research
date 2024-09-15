use crate::integrations::Integration;
use crate::runtime::device::{Device, DeviceType};
use crate::runtime::event::{Event, EventType};
use async_trait::async_trait;
use std::time::{Duration, Instant};
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
            id: "dummy-device-2707".into(),
            name: "Dummy Device".into(),
            typ: DeviceType::Dummy,
        }]
        .into()
    }

    fn subscribe(&self) -> UnboundedReceiver<Event> {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            loop {
                let result = tx.send(Event {
                    typ: EventType::Dummy,
                    time: Instant::now(),
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
