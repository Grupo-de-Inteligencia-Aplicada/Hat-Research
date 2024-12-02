use crate::runtime::device::Device;
use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::device::DeviceType;

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
    pub const fn get_events_related_to(device_type: DeviceType) -> &'static [EventType] {
        use EventType::*;
        match device_type {
            DeviceType::Dummy => &[Dummy],
            DeviceType::DoorSensor => &[DoorOpenEvent, DoorCloseEvent],
            DeviceType::Light => &[LightOnEvent, LightOffEvent],
            DeviceType::Sensor => &[SensorValueChangeEvent],
            DeviceType::PowerOutlet => &[PowerOutletOnEvent, PowerOutletOffEvent],
            DeviceType::MotionSensor => &[MotionSensorOnEvent, MotionSensorOffEvent],
            DeviceType::Unknown => &[],
        }
    }
    pub const fn as_str(&self) -> &'static str {
        use EventType::*;
        match self {
            Dummy => "Dummy",
            DoorOpenEvent => "DoorOpenEvent",
            DoorCloseEvent => "DoorCloseEvent",
            LightOnEvent => "LightOnEvent",
            LightOffEvent => "LightOffEvent",
            PowerOutletOnEvent => "PowerOutletOnEvent",
            PowerOutletOffEvent => "PowerOutletOffEvent",
            MotionSensorOnEvent => "MotionSensorOnEvent",
            MotionSensorOffEvent => "MotionSensorOffEvent",
            SensorValueChangeEvent => "SensorValueChangeEvent",
        }
    }
    pub const fn get_description(&self) -> &'static str {
        use EventType::*;
        match self {
            Dummy => "Dummy",
            DoorOpenEvent => "Door opened",
            DoorCloseEvent => "Door closed",
            LightOnEvent => "Light turned on",
            LightOffEvent => "Light turned off",
            PowerOutletOnEvent => "Power outlet turned on",
            PowerOutletOffEvent => "Power outlet turned off",
            MotionSensorOnEvent => "Movement detected",
            MotionSensorOffEvent => "Movement not detected",
            SensorValueChangeEvent => "Sensor value updated",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub typ: EventType,
    pub datetime: chrono::DateTime<Local>,
    pub device: Device,
    pub parameters: HashMap<String, String>,
}
