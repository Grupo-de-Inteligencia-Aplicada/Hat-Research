use std::sync::Arc;

use crate::runtime::value::Value;
use crate::runtime::HatRuntime;
use crate::runtime::{function::Function, value::time::Time};
use anyhow::{anyhow, bail, Context};
use lazy_static::lazy_static;
use tracing::{error, info};

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
                fun: |ctx, _args| Ok(Time::from(ctx.event.datetime).into()),
            },
            Function {
                name: "time".to_owned(),
                fun: |_ctx, args| {
                    let arg = args.first();
                    match arg {
                        Some(arg) => {
                            if let Value::String(s) = arg {
                                let mut parts = s.split(":");
                                let hours: u32 = parts
                                    .next()
                                    .context("time string is empty")?
                                    .parse()
                                    .context("failed to parse hours")?;
                                let mins: u32 = parts
                                    .next()
                                    .map(|s| s.parse::<u32>())
                                    .unwrap_or(Ok(0))
                                    .context("failed to parse minutes")?;
                                let secs: u32 = parts
                                    .next()
                                    .map(|s| s.parse::<u32>())
                                    .unwrap_or(Ok(0))
                                    .context("failed to parse seconds")?;
                                Ok(Value::Time(
                                    Time::from_hms_opt(hours, mins, secs)
                                        .context("invalid time provided")?,
                                ))
                            } else {
                                bail!("time function only accepts strings");
                            }
                        }
                        None => Ok(Value::Time(Time::now())),
                    }
                },
            },
            Function {
                name: "turn_off_device".to_owned(),
                fun: |ctx, args| {
                    let full_device_id = {
                        let first = args
                            .first()
                            .ok_or(anyhow!("missing device_id on turn_off_device function"))?;
                        if let Value::String(arg) = first {
                            arg.clone()
                        } else {
                            bail!("device id must be a string")
                        }
                    };

                    let runtime = Arc::clone(&ctx.runtime);

                    tokio::spawn(async move {
                        let (integration, device_id) =
                            HatRuntime::parse_full_device_id(&full_device_id);

                        if let Some(integration) = integration {
                            match runtime.get_integration(integration).await {
                                Some(integration) => {
                                    if let Err(e) = integration.turn_off_device(device_id).await {
                                        error!("failed to turn off device {full_device_id}: {e:?}");
                                    }
                                }
                                None => {
                                    error!("failed to find integration of device {full_device_id}");
                                }
                            }
                        } else {
                            todo!()
                        }
                    });

                    Ok(Value::Null)
                },
            },
            Function {
                name: "turn_on_device".to_owned(),
                fun: |ctx, args| {
                    let full_device_id = {
                        let first = args
                            .first()
                            .ok_or(anyhow!("missing device_id on turn_on_device function"))?;
                        if let Value::String(arg) = first {
                            arg.clone()
                        } else {
                            bail!("device id must be a string")
                        }
                    };

                    let runtime = Arc::clone(&ctx.runtime);

                    tokio::spawn(async move {
                        let (integration, device_id) =
                            HatRuntime::parse_full_device_id(&full_device_id);

                        if let Some(integration) = integration {
                            match runtime.get_integration(integration).await {
                                Some(integration) => {
                                    if let Err(e) = integration.turn_on_device(device_id).await {
                                        error!("failed to turn on device {full_device_id}: {e:?}");
                                    }
                                }
                                None => {
                                    error!("failed to find integration of device {full_device_id}");
                                }
                            }
                        } else {
                            todo!()
                        }
                    });

                    Ok(Value::Null)
                },
            },
        ]
    };
}
