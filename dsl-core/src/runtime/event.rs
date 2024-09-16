use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::runtime::device::Device;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Dummy,
    DoorOpenEvent,
    DoorCloseEvent,
    LightOnEvent,
    LightOffEvent,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Dummy => "Dummy",
            EventType::DoorOpenEvent => "DoorOpenEvent",
            EventType::DoorCloseEvent => "DoorCloseEvent",
            EventType::LightOnEvent => "LightOnEvent",
            EventType::LightOffEvent => "LightOffEvent",
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub typ: EventType,
    pub time: chrono::DateTime<Utc>,
    pub device: Device,
}
