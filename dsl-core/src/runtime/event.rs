use std::collections::HashMap;
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
    SensorValueChangeEvent,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Dummy => "Dummy",
            EventType::DoorOpenEvent => "DoorOpenEvent",
            EventType::DoorCloseEvent => "DoorCloseEvent",
            EventType::LightOnEvent => "LightOnEvent",
            EventType::LightOffEvent => "LightOffEvent",
            EventType::SensorValueChangeEvent => "SensorValueChangeEvent",
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub typ: EventType,
    pub time: chrono::DateTime<Utc>,
    pub device: Device,
    pub parameters: HashMap<String, String>,
}
