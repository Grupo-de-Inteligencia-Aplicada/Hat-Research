#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    Dummy,
}

pub struct Device {
    pub id: String,
    pub name: String,
    pub typ: DeviceType,
}
