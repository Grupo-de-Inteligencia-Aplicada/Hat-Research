use crate::runtime::expression::Expression;
use super::{actions::Action, event::Event, HatRuntime};

#[derive(Debug)]
pub struct Automation {
    pub name: String,
    pub triggers: Vec<String>,
    pub actions: Vec<Box<dyn Action>>,
}

impl Automation {
    pub fn should_be_triggered_by(&self, event: &Event) -> bool {
        for trigger in &self.triggers {
            if event.typ.as_str().to_lowercase() == trigger.to_lowercase() {
                return true;
            }
        }
        false
    }

    pub fn trigger(&self, runtime: &HatRuntime) {
        for action in &self.actions {
            action.run(runtime);
        }
    }
}
