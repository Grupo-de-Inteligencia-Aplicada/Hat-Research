use crate::runtime::device::Device;
use async_trait::async_trait;

pub mod home_assistant;

#[async_trait]
pub trait Integration {
    async fn list_devices(&self) -> Vec<Device>;
}
