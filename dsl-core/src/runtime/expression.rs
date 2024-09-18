use crate::runtime::context::AutomationContext;
use crate::runtime::function::FunctionCall;
use crate::runtime::value::Value;

use anyhow::Result;

#[derive(Debug)]
pub enum Expression {
    Constant(Value),
    Function(FunctionCall),
}

impl Expression {
    pub fn evaluate(&self, ctx: &mut AutomationContext) -> Result<Value> {
        match self {
            Expression::Constant(value) => Ok(value.clone()),
            Expression::Function(function) => function.evaluate(ctx),
        }
    }
}

impl From<Value> for Expression {
    fn from(value: Value) -> Self {
        Self::Constant(value)
    }
}

impl From<FunctionCall> for Expression {
    fn from(value: FunctionCall) -> Self {
        Self::Function(value)
    }
}
