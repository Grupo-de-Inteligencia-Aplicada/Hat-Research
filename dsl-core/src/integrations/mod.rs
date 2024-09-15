use crate::runtime::device::Device;
use async_trait::async_trait;
use tokio::sync::mpsc;
use crate::runtime::event::Event;

pub mod home_assistant;
pub mod dummy;

#[async_trait]
pub trait Integration: Send + Sync {
    async fn list_devices(&self) -> Vec<Device>;
    fn subscribe(&self) -> mpsc::UnboundedReceiver<Event>;
    fn name(&self) -> &'static str;
}
