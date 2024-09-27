use super::event::Event;
use crate::runtime::context::AutomationContext;
use crate::runtime::parser::expression::Expression;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Automation {
    pub name: String,
    pub triggers: Vec<String>,
    pub conditions: Vec<Expression>,
    pub actions: Vec<Expression>,
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

    pub fn trigger(&self, ctx: &mut AutomationContext) -> Result<()> {
        for condition in &self.conditions {
            let result = condition.evaluate(ctx).with_context(|| {
                format!("failed to evaluate expression in condition {condition:?}")
            })?;

            if !result.as_bool() {
                return Ok(());
            }
        }
        for action in &self.actions {
            action.evaluate(ctx)
                .with_context(|| format!("action of automation {} failed", self.name))?;
        }
        Ok(())
    }
}
