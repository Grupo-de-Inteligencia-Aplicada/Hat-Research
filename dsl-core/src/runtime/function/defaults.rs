use crate::runtime::function::Function;
use crate::runtime::value::Value;
use anyhow::{bail, Context};
use chrono::{NaiveTime, Utc};
use lazy_static::lazy_static;
use tracing::{debug, info};

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
            Function {
                name: "time".to_owned(),
                fun: |_ctx, args| {
                    let arg = args.first();
                    match arg {
                        Some(arg) => {
                            if let Value::String(s) = arg {
                                let mut parts = s.split(":");
                                let hours: u32 = parts.next()
                                    .context("time string is empty")?
                                    .parse()
                                    .context("failed to parse hours")?;
                                let mins: u32 = parts.next()
                                    .map(|s| s.parse::<u32>())
                                    .unwrap_or(Ok(0))
                                    .context("failed to parse minutes")?;
                                let secs: u32 = parts.next()
                                    .map(|s| s.parse::<u32>())
                                    .unwrap_or(Ok(0))
                                    .context("failed to parse seconds")?;
                                Ok(Value::Time(NaiveTime::from_hms_opt(hours, mins, secs).context("invalid time provided")?))
                            } else {
                                bail!("time function only accepts strings");
                            }
                        },
                        None => {
                            Ok(Value::Time(Utc::now().time()))
                        }
                    }
                },
            },
        ]
    };
}
