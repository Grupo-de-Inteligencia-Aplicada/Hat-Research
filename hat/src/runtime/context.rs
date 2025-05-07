use crate::runtime::event::Event;
use crate::runtime::function::Function;
use crate::runtime::HatRuntime;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use super::scheduler::TaskID;

pub struct ExpressionContext {
    pub trigger: Trigger,
    pub runtime: Arc<HatRuntime>,
}

#[derive(Debug)]
pub enum Trigger {
    Event(Event),
    Task(TaskID),
}

impl Debug for ExpressionContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutomationContext")
            .field("trigger", &self.trigger)
            .finish()
    }
}

impl ExpressionContext {
    pub fn get_function(&self, name: &str) -> Option<Arc<Function>> {
        self.runtime
            .functions
            .read()
            .unwrap()
            .get(name)
            .map(Arc::clone)
    }
}
