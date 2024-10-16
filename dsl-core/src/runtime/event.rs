use crate::runtime::device::Device;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Dummy,
    DoorOpenEvent,
    DoorCloseEvent,
    LightOnEvent,
    LightOffEvent,
    PowerOutletOnEvent,
    PowerOutletOffEvent,
    MotionSensorOnEvent,
    MotionSensorOffEvent,
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
            EventType::PowerOutletOnEvent => "PowerOutletOnEvent",
            EventType::PowerOutletOffEvent => "PowerOutletOffEvent",
            EventType::MotionSensorOnEvent => "MotionSensorOnEvent",
            EventType::MotionSensorOffEvent => "MotionSensorOffEvent",
            EventType::SensorValueChangeEvent => "SensorValueChangeEvent",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub typ: EventType,
    pub datetime: chrono::DateTime<Utc>,
    pub device: Device,
    pub parameters: HashMap<String, String>,
}
