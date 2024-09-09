use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub enum EventType {
    Dummy,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Dummy => "dummy",
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub typ: EventType,
    pub time: Instant,
}
