pub mod defaults;

use crate::runtime::context::AutomationContext;
use crate::runtime::parser::expression::Expression;
use crate::runtime::value::Value;

use anyhow::{bail, Result};

pub(crate) type NativeFunctionType = fn(&mut AutomationContext, Vec<Value>) -> Result<Value>;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub fun: NativeFunctionType,
}

impl Function {
    pub fn call(&self, ctx: &mut AutomationContext, args: Vec<Value>) -> Result<Value> {
        (self.fun)(ctx, args)
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn evaluate(&self, ctx: &mut AutomationContext) -> Result<Value> {
        let arguments = self
            .arguments
            .iter()
            .map(|expr| expr.evaluate(ctx))
            .collect::<Result<Vec<Value>>>()?;

        let fun = ctx.get_function(&self.name);

        match fun {
            Some(fun) => fun.call(ctx, arguments),
            None => bail!("function {} not found!", self.name),
        }
    }
}
