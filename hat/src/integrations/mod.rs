use crate::runtime::device::Device;
use crate::runtime::event::Event;
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

pub mod dummy;
pub mod home_assistant;
pub(crate) mod clock;

#[async_trait]
pub trait Integration: Send + Sync {
    async fn list_devices(&self) -> Result<Vec<Device>>;
    async fn get_device(&self, id: &str) -> Result<Option<Device>>;
    async fn turn_on_device(&self, device_id: &str) -> Result<()>;
    async fn turn_off_device(&self, device_id: &str) -> Result<()>;
    async fn set_light_color_rgb(&self, device_id: &str, color: [u8; 3]) -> Result<()>;
    async fn set_light_brightness(&self, device_id: &str, brightness: u8) -> Result<()>;
    fn subscribe(&self) -> mpsc::UnboundedReceiver<Event>;
    fn get_id(&self) -> &str;
}
