use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

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
    pub fn evaluate<'a>(
        &'a self,
        ctx: Arc<AutomationContext>,
    ) -> Pin<Box<dyn Future<Output = Result<Value>> + Send + 'a>> {
        Box::pin(async move {
            match self {
                Expression::Constant(value) => Ok(value.clone()),
                Expression::Function(function) => function.evaluate(ctx).await,
                Expression::BinaryOperation { lhs, op, rhs } => {
                    // Box recursive calls
                    let lh_value = lhs.evaluate(Arc::clone(&ctx)).await?;
                    let rh_value = rhs.evaluate(ctx).await?;
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
        })
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(c) => write!(f, "{c}"),
            Self::Function(fun) => write!(f, "{fun}"),
            Self::BinaryOperation { lhs, op, rhs } => {
                write!(f, "{} {} {}", lhs, op, rhs)
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
