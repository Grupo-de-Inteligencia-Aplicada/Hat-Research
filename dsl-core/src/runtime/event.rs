use crate::runtime::device::Device;
use chrono::Local;
use serde::{Deserialize, Serialize};
use strum::VariantArray;
use std::collections::HashMap;

use super::device::DeviceType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Dummy,
    DoorOpenEvent,
    DoorCloseEvent,
    LightOnEvent,
    LightOffEvent,
    SwitchTurnedOnEvent,
    SwitchTurnedOffEvent,
    MotionSensorOnEvent,
    MotionSensorOffEvent,
    SensorValueChangeEvent,
    ClockTickEvent,
    ButtonPressedEvent,
}

impl EventType {
    pub const fn get_events_related_to(device_type: DeviceType) -> &'static [EventType] {
        use EventType::*;
        match device_type {
            DeviceType::Dummy => &[Dummy],
            DeviceType::DoorSensor => &[DoorOpenEvent, DoorCloseEvent],
            DeviceType::Light => &[LightOnEvent, LightOffEvent],
            DeviceType::Sensor => &[SensorValueChangeEvent],
            DeviceType::Switch => &[SwitchTurnedOnEvent, SwitchTurnedOffEvent],
            DeviceType::MotionSensor => &[MotionSensorOnEvent, MotionSensorOffEvent],
            DeviceType::Button => &[ButtonPressedEvent],
            DeviceType::Unknown => &[],
        }
    }
    pub fn get_related_device_type(&self) -> Option<DeviceType> {
        for dt in DeviceType::VARIANTS {
            let events = Self::get_events_related_to(*dt);
            for e in events {
                if e == self {
                    return Some(*dt);
                }
            }
        }
        return None;
    }
    pub const fn as_str(&self) -> &'static str {
        use EventType::*;
        match self {
            Dummy => "Dummy",
            DoorOpenEvent => "DoorOpenEvent",
            DoorCloseEvent => "DoorCloseEvent",
            LightOnEvent => "LightOnEvent",
            LightOffEvent => "LightOffEvent",
            SwitchTurnedOnEvent => "SwitchTurnedOnEvent",
            SwitchTurnedOffEvent => "SwitchTurnedOffEvent",
            MotionSensorOnEvent => "MotionSensorOnEvent",
            MotionSensorOffEvent => "MotionSensorOffEvent",
            SensorValueChangeEvent => "SensorValueChangeEvent",
            ClockTickEvent => "ClockTickEvent",
            ButtonPressedEvent => "ButtonPressedEvent",
        }
    }
    pub const fn get_description(&self) -> &'static str {
        use EventType::*;
        match self {
            Dummy => "Dummy",
            DoorOpenEvent => "Porta abriu",
            DoorCloseEvent => "Porta fechou",
            LightOnEvent => "Luz foi ligada",
            LightOffEvent => "Luz foi apagada",
            SwitchTurnedOnEvent => "Interruptor foi ligado",
            SwitchTurnedOffEvent => "Interruptor foi desligado",
            MotionSensorOnEvent => "Movimento foi detectado",
            MotionSensorOffEvent => "Movimento não é mais detectado",
            SensorValueChangeEvent => "Valor de sensor mudou",
            ClockTickEvent => "Run every second",
            ButtonPressedEvent => "Botão foi apertado",
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
