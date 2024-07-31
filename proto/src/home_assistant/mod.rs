pub mod command;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use anyhow::{Context, ensure, Result};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message as WebSocketMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::debug;
use command::Command;
use tokio::sync::{mpsc, Mutex};

#[derive(Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub msg_type: String,
}

pub struct HAWebSocket {
    pub ha_version: String,
    pub tx: Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WebSocketMessage>>,
    last_command_id: AtomicUsize,
    command_channels: Arc<Mutex<HashMap<usize, mpsc::Sender<Message>>>>,
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
        
        tokio::spawn(async move {
            
        });

        Ok(Self {
            ha_version: ha_version.to_owned(),
            tx: Mutex::new(tx),
            last_command_id: AtomicUsize::default(),
            command_channels: Arc::new(Mutex::new(HashMap::default())),
        })
    }
    pub async fn ping(&self, timeout: Duration) -> Result<()> {
        // let id = self.generate_message_id();
        // let msg = Message {
        //     id,
        //     msg_type: "ping".to_owned(),
        // };
        todo!()
    }
    fn generate_command_id(&self) -> usize {
        self.last_command_id.fetch_add(1, Ordering::SeqCst)
    }
}

