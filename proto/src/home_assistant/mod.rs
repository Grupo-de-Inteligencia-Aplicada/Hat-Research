pub mod command;

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use anyhow::{Context, ensure, Result};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message as WebSocketMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{debug, warn};
use command::Command;
use tokio::sync::{mpsc, Mutex};
use tokio::time::timeout;
use crate::home_assistant::command::CommandMessage;

#[derive(Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub msg_type: String,
}

pub struct HAWebSocket {
    pub ha_version: String,
    pub tx: Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WebSocketMessage>>,
    last_command_id: AtomicUsize,
    command_channels: Arc<Mutex<BTreeMap<usize, mpsc::Sender<Message>>>>,
}

impl HAWebSocket {
    pub async fn new_command(&self) -> Command {
        let id = self.generate_command_id();
        let (tx, rx) = mpsc::channel(3);
        let cmd = Command {
            ws: self,
            id,
            recv: rx,
        };
        let mut channels = self.command_channels.lock().await;
        channels.insert(id, tx);
        cmd
    }
    async fn drop_command(&self, command_id: usize) {
        let mut channels = self.command_channels.lock().await;
        channels.remove(&command_id);
    }
    pub async fn connect(url: &str, access_token: &str) -> Result<Self> {
        debug!("Connecting to websocket at {url}...");

        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        let (mut tx, mut rx) = ws_stream.split();

        let msg = rx.next().await
            .context("expected websocket message")?
            .context("failed to read from websocket")?
            .into_text().context("failed to convert message to text")?;
        let msg = serde_json::from_str::<serde_json::Value>(&msg)
            .context("failed to parse message json")?;
        let msg_obj = msg.as_object()
            .context("message is not an object")?;

        let ha_version = msg_obj
            .get("ha_version").context("server sent no version")?
            .as_str().context("server sent invalid version")?;
        let msg_type = msg_obj
            .get("type").context("message has no type")?
            .as_str().context("server sent invalid message type")?;

        ensure!(msg_type == "auth_required", "first message must be auth");

        let auth_response = json!({
            "type": "auth",
            "access_token": access_token,
        });

        tx.send(WebSocketMessage::Text(serde_json::to_string(&auth_response)?)).await?;

        let msg = rx.next().await
            .context("expected websocket message")?
            .context("failed to read from websocket")?
            .into_text().context("failed to convert message to text")?;
        let msg = serde_json::from_str::<serde_json::Value>(&msg)
            .context("failed to parse message json")?;
        let msg_obj = msg.as_object()
            .context("message is not an object")?;

        let msg_type = msg_obj.get("type").map(|t| t.as_str());
        ensure!(msg_type == Some(Some("auth_ok")), "authentication failed");

        debug!("Connected and authenticated to HomeAssistant {ha_version}!");

        let command_channels = Arc::new(
            Mutex::new(BTreeMap::<usize, mpsc::Sender<Message>>::new())
        );

        let channels = Arc::clone(&command_channels);
        
        tokio::spawn(async move {
            while let Some(msg) = rx.next().await {
                match msg {
                    Ok(msg) => {
                        let json = match msg.into_text() {
                            Ok(msg) => msg,
                            Err(_) => { continue; },
                        };
                        debug!("Received: {json}");
                        let msg: CommandMessage = match serde_json::from_str(&json) {
                            Ok(msg) => msg,
                            Err(e) => {
                                error!("Failed to parse message from home assistant: {e:?}");
                                continue;
                            }
                        };
                        let channel = {
                            let channels_lock = channels.lock().await;
                            channels_lock.get(&msg.id).map(|c| c.clone())
                        };
                        match channel {
                            Some(mut c) => {
                                c.send(msg.message).await.ok();
                            },
                            None => warn!("Received message for unknown command channel {}", msg.id),
                        }
                    },
                    Err(e) => {
                        error!("Failed to read message from home assistant: {e:?}");
                    }
                }
            }
        });

        Ok(Self {
            ha_version: ha_version.to_owned(),
            tx: Mutex::new(tx),
            last_command_id: AtomicUsize::new(1),
            command_channels,
        })
    }
    pub async fn ping(&self, timeout: Duration) -> Result<()> {
        let mut command = self.new_command().await;
        command.send_message(Message {
            msg_type: "ping".into()
        }).await?;
        let res = tokio::time::timeout(timeout, command.receive_message()).await
            .context("ping command timed out")??;
        ensure!(&res.msg_type == "pong");
        Ok(())
    }
    fn generate_command_id(&self) -> usize {
        self.last_command_id.fetch_add(1, Ordering::SeqCst)
    }
}

