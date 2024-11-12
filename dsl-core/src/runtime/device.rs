use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum DeviceType {
    Dummy,
    DoorSensor,
    Light,
    Sensor,
    PowerOutlet,
    MotionSensor,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    pub integration: String,
    pub id: String,
    pub name: Option<String>,
    pub typ: DeviceType,
    pub state: Option<String>,
}

impl Device {
    pub fn full_id(&self) -> String {
        format!("{}@{}", self.integration, self.id)
    }
}
