use crate::runtime::event::Event;
use crate::runtime::function::Function;
use crate::runtime::HatRuntime;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct AutomationContext<'r> {
    pub event: Event,
    pub runtime: &'r HatRuntime,
}

impl Debug for AutomationContext<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutomationContext")
            .field("event", &self.event)
            .finish()
    }
}

impl<'r> AutomationContext<'r> {
    pub fn get_function(&self, name: &str) -> Option<Arc<Function>> {
        self.runtime
            .functions
            .read()
            .unwrap()
            .get(name)
            .map(Arc::clone)
    }
}
