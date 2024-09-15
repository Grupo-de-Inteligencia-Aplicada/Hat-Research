use super::HAWebSocket;
use crate::runtime::event::Event;
use crate::{integrations::Integration, runtime::device::Device};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct HassIntegration {
    ws: HAWebSocket,
}

impl HassIntegration {
    pub async fn new(hass_url: &str, access_token: &str) -> Result<Self> {
        let ws = HAWebSocket::connect(hass_url, access_token).await?;
        Ok(Self { ws })
    }
}

#[async_trait]
impl Integration for HassIntegration {
    async fn list_devices(&self) -> Vec<Device> {
        todo!()
    }

    fn subscribe(&self) -> UnboundedReceiver<Event> {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}
