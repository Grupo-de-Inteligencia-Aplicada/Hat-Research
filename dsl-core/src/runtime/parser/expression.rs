use crate::runtime::context::AutomationContext;
use crate::runtime::function::FunctionCall;
use crate::runtime::value::Value;

use crate::runtime::parser::operation::Operation;
use crate::runtime::value::operations::{TryAdd, TryDiv, TryMul, TrySub};
use anyhow::Result;

#[derive(Debug)]
pub enum Expression {
    Constant(Value),
    Function(FunctionCall),
    BinaryOperation {
        lhs: Box<Expression>,
        op: Operation,
        rhs: Box<Expression>,
    },
}

impl Expression {
    pub fn evaluate(&self, ctx: &mut AutomationContext) -> Result<Value> {
        match self {
            Expression::Constant(value) => Ok(value.clone()),
            Expression::Function(function) => function.evaluate(ctx),
            Expression::BinaryOperation { lhs, op, rhs } => {
                let lh_value = lhs.evaluate(ctx)?;
                let rh_value = rhs.evaluate(ctx)?;
                match op {
                    Operation::Add => lh_value.try_add(rh_value),
                    Operation::Subtract => lh_value.try_sub(rh_value),
                    Operation::Multiply => lh_value.try_mul(rh_value),
                    Operation::Divide => lh_value.try_div(rh_value),
                    Operation::Equals => Ok(Value::Boolean(lh_value == rh_value)),
                    Operation::NotEquals => Ok(Value::Boolean(lh_value != rh_value)),
                    Operation::And => Ok(Value::Boolean(lh_value.as_bool() && rh_value.as_bool())),
                    Operation::Or => Ok(Value::Boolean(lh_value.as_bool() || rh_value.as_bool())),
                    Operation::Greater => Ok(Value::Boolean(lh_value > rh_value)),
                    Operation::GreaterOrEquals => Ok(Value::Boolean(lh_value >= rh_value)),
                    Operation::Lesser => Ok(Value::Boolean(lh_value < rh_value)),
                    Operation::LesserOrEquals => Ok(Value::Boolean(lh_value <= rh_value)),
                }
            }
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
