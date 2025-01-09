use std::{collections::HashMap, time::Duration};

use crate::runtime::{
    device::{Device, DeviceType},
    event::{Event, EventType},
};
use anyhow::{bail, Result};
use chrono::Local;
use tokio::sync::mpsc;

use super::Integration;

pub struct ClockIntegration;

#[async_trait::async_trait]
impl Integration for ClockIntegration {
    async fn list_devices(&self) -> Result<Vec<Device>> {
        return Ok(Vec::new());
    }

    async fn get_device(&self, _: &str) -> Result<Option<Device>> {
        return Ok(None);
    }

    async fn turn_on_device(&self, _: &str) -> Result<()> {
        bail!("ClockIntegration has no devices to turn on");
    }

    async fn turn_off_device(&self, _: &str) -> Result<()> {
        bail!("ClockIntegration has no devices to turn off");
    }

    async fn set_light_color_rgb(&self, _: &str, _: [u8; 3]) -> Result<()> {
        bail!("ClockIntegration has no light to configure");
    }

    async fn set_light_brightness(&self, _: &str, _: u8) -> Result<()> {
        bail!("ClockIntegration has no light to configure");
    }

    fn subscribe(&self) -> mpsc::UnboundedReceiver<Event> {
        let (tx, rx) = mpsc::unbounded_channel();

        let integration = self.get_id().to_owned();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            let device = Device {
                integration: integration.clone(),
                id: "Clock".to_owned(),
                name: None,
                typ: DeviceType::Unknown,
                state: None,
                attributes: serde_json::Map::new(),
            };
            loop {
                interval.tick().await;

                let event = Event {
                    typ: EventType::ClockTickEvent,
                    datetime: Local::now(),
                    device: device.clone(),
                    parameters: HashMap::new(),
                };

                if tx.send(event).is_err() {
                    break;
                }
            }
        });

        rx
    }

    fn get_id(&self) -> &str {
        "ClockIntegration"
    }
}
