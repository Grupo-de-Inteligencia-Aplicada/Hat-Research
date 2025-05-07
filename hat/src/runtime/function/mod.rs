pub mod defaults;

use std::{fmt::Display, future::Future, pin::Pin, sync::Arc};

use crate::runtime::context::ExpressionContext;
use crate::runtime::parser::expression::Expression;
use crate::runtime::value::Value;

use anyhow::{bail, Context, Result};

pub(crate) type NativeFunctionType =
    fn(Arc<ExpressionContext>, Vec<Value>) -> Pin<Box<dyn Future<Output = Result<Value>> + Send>>;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub fun: NativeFunctionType,
}

impl Function {
    pub async fn call(&self, ctx: Arc<ExpressionContext>, args: Vec<Value>) -> Result<Value> {
        (self.fun)(ctx, args).await
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn evaluate<'a>(
        &'a self,
        ctx: Arc<ExpressionContext>,
    ) -> Pin<Box<dyn Future<Output = Result<Value>> + Send + 'a>> {
        Box::pin(async move {
            let mut arguments = Vec::with_capacity(self.arguments.len());

            for (idx, arg) in self.arguments.iter().enumerate() {
                let result = arg.evaluate(Arc::clone(&ctx)).await.with_context(|| {
                    format!(
                        "failed to evaluate argument {} of function {}",
                        idx + 1,
                        self.name
                    )
                })?;
                arguments.push(result);
            }

            let fun = ctx.get_function(&self.name);

            match fun {
                Some(fun) => fun.call(ctx, arguments).await,
                None => bail!("function {} not found!", self.name),
            }
        })
    }
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arguments = self
            .arguments
            .iter()
            .map(|expr| expr.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}({})", self.name, arguments,)
    }
}
