use crate::runtime::device::Device;
use crate::runtime::event::Event;
use async_trait::async_trait;
use tokio::sync::mpsc;

pub mod dummy;
pub mod home_assistant;

#[async_trait]
pub trait Integration: Send + Sync {
    async fn list_devices(&self) -> Vec<Device>;
    async fn get_device(&self, id: &str) -> Option<Device>;
    fn subscribe(&self) -> mpsc::UnboundedReceiver<Event>;
    fn get_id(&self) -> &str;
}
