use anyhow::{Context, Result};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message as WebSocketMessage;
use crate::home_assistant::{HAWebSocket, Message};
use tokio::sync::mpsc;
use tracing::debug;

#[derive(Deserialize, Serialize)]
pub(super) struct CommandMessage {
    pub id: usize,
    #[serde(flatten)]
    pub message: Message,
}

pub struct Command<'a> {
    pub(super) ws: &'a HAWebSocket,
    pub(super) id: usize,
    pub(super) recv: mpsc::Receiver<Message>,
}

impl<'a> Command<'a> {
    pub async fn send_message(&self, msg: Message) -> Result<()> {
        let command_message = CommandMessage {
            id: self.id,
            message: msg,
        };
        let json = serde_json::to_string(&command_message)?;
        let mut tx = self.ws.tx.lock().await;
        tx.send(WebSocketMessage::Text(json)).await?;
        Ok(())
    }
    pub async fn receive_message(&mut self) -> Result<Message> {
        self.recv.recv().await
            .context("command channel already closed")
    }
}

impl<'a> Drop for Command<'a> {
    fn drop(&mut self) {
    }
}
