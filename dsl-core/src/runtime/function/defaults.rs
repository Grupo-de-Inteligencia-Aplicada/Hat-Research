use std::sync::Arc;
use std::time::Duration;

use crate::runtime::value::Value;
use crate::runtime::HatRuntime;
use crate::runtime::{function::Function, value::time::Time};
use anyhow::{anyhow, bail, ensure, Context};
use lazy_static::lazy_static;
use tracing::{error, info};

lazy_static! {
    pub static ref DEFAULT_FUNCTIONS: Vec<Function> = {
        vec![
            Function {
                name: "echo".to_owned(),
                fun: |_ctx, args| {
                    Box::pin(async move {
                        let args = args
                            .into_iter()
                            .map(|arg| arg.to_string())
                            .collect::<Vec<_>>()
                            .join(" ");
                        info!("[ECHO] {args}");
                        Ok(Value::Null)
                    })
                },
            },
            Function {
                name: "get_device".to_owned(),
                fun: (|ctx, _args| Box::pin(async move { Ok(ctx.event.device.full_id().into()) })),
            },
            Function {
                name: "get_integration".to_owned(),
                fun: (|ctx, _args| {
                    Box::pin(async move { Ok(ctx.event.device.integration.clone().into()) })
                }),
            },
            Function {
                name: "event_date".to_owned(),
                fun: (|ctx, _args| {
                    Box::pin(async move { Ok(ctx.event.datetime.to_rfc3339().into()) })
                }),
            },
            Function {
                name: "event_time".to_owned(),
                fun: (|ctx, _args| {
                    Box::pin(async move { Ok(Time::from(ctx.event.datetime).into()) })
                }),
            },
            Function {
                name: "time".to_owned(),
                fun: (|_ctx, args| {
                    Box::pin(async move {
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
                    })
                }),
            },
            Function {
                name: "turn_off_device".to_owned(),
                fun: (|ctx, args| {
                    Box::pin(async move {
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
                                        if let Err(e) = integration.turn_off_device(device_id).await
                                        {
                                            error!(
                                                "failed to turn off device {full_device_id}: {e:?}"
                                            );
                                        }
                                    }
                                    None => {
                                        error!(
                                            "failed to find integration of device {full_device_id}"
                                        );
                                    }
                                }
                            } else {
                                todo!()
                            }
                        });

                        Ok(Value::Null)
                    })
                }),
            },
            Function {
                name: "turn_on_device".to_owned(),
                fun: (|ctx, args| {
                    Box::pin(async move {
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
                                        if let Err(e) = integration.turn_on_device(device_id).await
                                        {
                                            error!(
                                                "failed to turn on device {full_device_id}: {e:?}"
                                            );
                                        }
                                    }
                                    None => {
                                        error!(
                                            "failed to find integration of device {full_device_id}"
                                        );
                                    }
                                }
                            } else {
                                todo!()
                            }
                        });

                        Ok(Value::Null)
                    })
                }),
            },
            Function {
                name: "set_light_color".to_owned(),
                fun: (|ctx, args| {
                    Box::pin(async move {
                        let full_device_id = {
                            let first = args.first().ok_or(anyhow!("missing device_id"))?;
                            if let Value::String(arg) = first {
                                arg.clone()
                            } else {
                                bail!("device id must be a string")
                            }
                        };
                        let color: [u8; 3] = {
                            let rgb_string = args.get(1).context("missing color argument")?;
                            if let Value::String(rgb_string) = rgb_string {
                                ensure!(
                                    rgb_string.len() == 7 && rgb_string.starts_with("#"),
                                    "Invalid RGB string format. Expected format: #RRGGBB"
                                );

                                let r = u8::from_str_radix(&rgb_string[1..3], 16)?;
                                let g = u8::from_str_radix(&rgb_string[3..5], 16)?;
                                let b = u8::from_str_radix(&rgb_string[5..7], 16)?;

                                [r, g, b]
                            } else {
                                bail!("invalid color argument");
                            }
                        };

                        let runtime = Arc::clone(&ctx.runtime);

                        tokio::spawn(async move {
                            let (integration, device_id) =
                                HatRuntime::parse_full_device_id(&full_device_id);

                            if let Some(integration) = integration {
                                match runtime.get_integration(integration).await {
                                    Some(integration) => {
                                        if let Err(e) =
                                            integration.set_light_color_rgb(device_id, color).await
                                        {
                                            error!("failed to set color on device {full_device_id}: {e:?}");
                                        }
                                    }
                                    None => {
                                        error!(
                                            "failed to find integration of device {full_device_id}"
                                        );
                                    }
                                }
                            } else {
                                todo!()
                            }
                        });

                        Ok(Value::Null)
                    })
                }),
            },
            Function {
                name: "set_light_brightness".to_owned(),
                fun: (|ctx, args| {
                    Box::pin(async move {
                        let full_device_id = {
                            let first = args.first().ok_or(anyhow!("missing device_id"))?;
                            if let Value::String(arg) = first {
                                arg.clone()
                            } else {
                                bail!("device id must be a string")
                            }
                        };
                        let brightness: u8 = {
                            let rgb_string = args.get(1).context("missing brightness argument")?;
                            if let Value::Number(brightness) = rgb_string {
                                u8::try_from(*brightness as i64)?
                            } else {
                                bail!("invalid brightness argument");
                            }
                        };

                        let runtime = Arc::clone(&ctx.runtime);

                        tokio::spawn(async move {
                            let (integration, device_id) =
                                HatRuntime::parse_full_device_id(&full_device_id);

                            if let Some(integration) = integration {
                                match runtime.get_integration(integration).await {
                                    Some(integration) => {
                                        if let Err(e) = integration
                                            .set_light_brightness(device_id, brightness)
                                            .await
                                        {
                                            error!("failed to set brightness on device {full_device_id}: {e:?}");
                                        }
                                    }
                                    None => {
                                        error!(
                                            "failed to find integration of device {full_device_id}"
                                        );
                                    }
                                }
                            } else {
                                todo!()
                            }
                        });

                        Ok(Value::Null)
                    })
                }),
            },
            Function {
                name: "is_device_on".to_owned(),
                fun: (|ctx, args| {
                    Box::pin(async move {
                        if let Some(Value::String(arg)) = args.first() {
                            if let Some(dev) = ctx.runtime.get_device(arg).await? {
                                if matches!(dev.state.as_deref(), Some("on")) {
                                    Ok(Value::Boolean(true))
                                } else {
                                    Ok(Value::Boolean(false))
                                }
                            } else {
                                bail!("device {arg} not found!");
                            }
                        } else {
                            bail!("first argument must be the device id")
                        }
                    })
                }),
            },
            Function {
                name: "is_device_off".to_owned(),
                fun: (|ctx, args| {
                    Box::pin(async move {
                        if let Some(Value::String(arg)) = args.first() {
                            if let Some(dev) = ctx.runtime.get_device(arg).await? {
                                if matches!(dev.state.as_deref(), Some("off")) {
                                    Ok(Value::Boolean(true))
                                } else {
                                    Ok(Value::Boolean(false))
                                }
                            } else {
                                bail!("device {arg} not found!");
                            }
                        } else {
                            bail!("first argument must be the device id")
                        }
                    })
                }),
            },
            Function {
                name: "wait".to_owned(),
                fun: (|_ctx, args| {
                    Box::pin(async move {
                        if let Some(Value::Number(seconds)) = args.first() {
                            let millis = (seconds * 1000.0) as u64;
                            let duration = Duration::from_millis(millis);
                            tokio::time::sleep(duration).await;
                            Ok(Value::Null)
                        } else {
                            bail!("first argument must be the seconds to wait")
                        }
                    })
                }),
            },
            Function {
                name: "get_device_state".to_owned(),
                fun: (|ctx, args| {
                    Box::pin(async move {
                        if let Some(Value::String(arg)) = args.first() {
                            if let Some(dev) = ctx.runtime.get_device(arg).await? {
                                Ok(match dev.state {
                                    Some(state) => Value::String(state),
                                    None => Value::Null,
                                })
                            } else {
                                bail!("device {arg} not found!");
                            }
                        } else {
                            bail!("first argument must be the device id")
                        }
                    })
                }),
            },
            Function {
                name: "number".to_owned(),
                fun: (|_, args| {
                    Box::pin(async move {
                        if let Some(arg) = args.first() {
                            match arg {
                                Value::String(arg) => Ok(Value::Number(arg.parse::<f64>()?)),
                                Value::Boolean(arg) => Ok(Value::Number(if *arg { 1f64 } else { 0f64 })),
                                Value::Number(n) => Ok(Value::Number(*n)),
                                Value::Time(_) => bail!("cannot convert time into a number"),
                                Value::Null => bail!("cannot convert null into a number"),
                            }
                        } else {
                            bail!("first argument is missing")
                        }
                    })
                }),
            },
        ]
    };
}
