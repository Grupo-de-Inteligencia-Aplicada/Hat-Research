use crate::runtime::function::Function;
use crate::runtime::value::Value;
use lazy_static::lazy_static;
use tracing::info;

lazy_static! {
    pub static ref DEFAULT_FUNCTIONS: Vec<Function> = {
        vec![
            Function {
                name: "echo".to_owned(),
                fun: |_ctx, args| {
                    let args = args
                        .into_iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<_>>()
                        .join(" ");
                    info!("[ECHO] {args}");
                    Ok(Value::Null)
                },
            },
            Function {
                name: "get_device".to_owned(),
                fun: |ctx, _args| Ok(ctx.event.device.full_id().into()),
            },
            Function {
                name: "get_integration".to_owned(),
                fun: |ctx, _args| Ok(ctx.event.device.integration.clone().into()),
            },
            Function {
                name: "event_date".to_owned(),
                fun: |ctx, _args| Ok(ctx.event.datetime.to_rfc3339().into()),
            },
            Function {
                name: "event_time".to_owned(),
                fun: |ctx, _args| Ok(ctx.event.datetime.time().into()),
            },
        ]
    };
}
