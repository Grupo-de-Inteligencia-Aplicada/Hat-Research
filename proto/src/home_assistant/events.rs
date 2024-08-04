use anyhow::{Context, ensure, Result};
use std::pin::Pin;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use crate::home_assistant::command::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    event_type: String,
    time_fired: String,
    origin: String,
    context: serde_json::Value,
    data: serde_json::Value,
}

pub struct Events<'a> {
    pub(super) command: Command<'a>,
}

impl<'a> Events<'a> {
    pub async fn next(&mut self) -> Result<Event> {
        let mut msg = self.command.receive_message().await?;
        ensure!(&msg.msg_type == "event");
        let event = msg.fields.remove("event")
            .context("message does not have event field")?;
        let event: Event = serde_json::from_value(event)?;
        Ok(event)
    }
}