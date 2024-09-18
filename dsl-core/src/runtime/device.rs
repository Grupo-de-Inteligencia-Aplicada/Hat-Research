use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DeviceType {
    Dummy,
    DoorSensor,
    Light,
    Sensor,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    pub integration: String,
    pub id: String,
    pub name: Option<String>,
    pub typ: DeviceType,
}

impl Device {
    pub fn full_id(&self) -> String {
        format!("{}@{}", self.integration, self.id)
    }
}
