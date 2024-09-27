use tracing::info;
use crate::runtime::HatRuntime;

use super::Action;

#[derive(Debug)]
pub struct EchoAction {
    message: String,
}

impl EchoAction {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Action for EchoAction {
    fn get_action_name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, _runtime: &HatRuntime) {
        info!("ECHO: {}", self.message);
    }
}
