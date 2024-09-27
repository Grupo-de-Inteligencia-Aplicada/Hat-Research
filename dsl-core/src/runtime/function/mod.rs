use crate::runtime::context::AutomationContext;
use crate::runtime::parser::expression::Expression;
use crate::runtime::value::Value;

use anyhow::{bail, Result};

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

        match &*self.name {
            "get_device" => default_functions::get_device(ctx, arguments),
            "get_integration" => default_functions::get_integration(ctx, arguments),
            "get_time" => default_functions::get_time(ctx, arguments),
            "is_device" => default_functions::is_device(ctx, arguments),
            _ => bail!("Unknown function {}!", &self.name),
        }
    }
}

pub mod default_functions {
    use crate::runtime::context::AutomationContext;
    use crate::runtime::value::Value;
    use anyhow::Result;

    pub fn get_device(ctx: &mut AutomationContext, _args: Vec<Value>) -> Result<Value> {
        Ok(ctx.event.device.full_id().into())
    }

    pub fn get_integration(ctx: &mut AutomationContext, _args: Vec<Value>) -> Result<Value> {
        Ok(ctx.event.device.integration.clone().into())
    }

    pub fn get_time(ctx: &mut AutomationContext, _args: Vec<Value>) -> Result<Value> {
        Ok(ctx.event.time.to_rfc3339().into())
    }

    pub fn is_device(ctx: &mut AutomationContext, args: Vec<Value>) -> Result<Value> {
        let device = &ctx.event.device.id;
        let first_arg = args.first().map(|v| v.to_string());
        Ok(Value::from(Some(device) == first_arg.as_ref()))
    }
}
