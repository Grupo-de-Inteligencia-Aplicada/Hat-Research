use anyhow::Result;
use async_trait::async_trait;

use crate::{integrations::Integration, runtime::device::Device};

use super::HAWebSocket;

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
}
