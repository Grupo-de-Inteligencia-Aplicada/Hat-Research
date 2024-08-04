use crate::home_assistant::command::Command;
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_type: String,
    pub time_fired: String,
    pub origin: String,
    pub context: serde_json::Value,
    pub data: EventData,
}

pub struct Events<'a> {
    pub(super) command: Command<'a>,
}

impl<'a> Events<'a> {
    pub async fn next(&mut self) -> Result<Event> {
        let mut msg = self.command.receive_message().await?;
        ensure!(&msg.msg_type == "event");
        let event = msg
            .fields
            .remove("event")
            .context("message does not have event field")?;
        let event: Event = serde_json::from_value(event)?;
        Ok(event)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    StateChanged {
        entity_id: String,
        new_state: Map<String, Value>,
        old_state: Map<String, Value>,
    },
    Unknown {
        #[serde(flatten)]
        data: serde_json::Value,
    },
}
