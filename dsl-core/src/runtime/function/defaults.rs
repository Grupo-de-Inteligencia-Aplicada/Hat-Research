use lazy_static::lazy_static;

use crate::runtime::function::Function;

lazy_static! {

    pub static ref DEFAULT_FUNCTIONS: Vec<Function> = {
        vec![
            Function {
                name: "get_device".to_owned(),
                fun: |ctx, args| {
                    Ok(ctx.event.device.full_id().into())
                },
            },
            Function {
                name: "get_integration".to_owned(),
                fun: |ctx, args| {
                    Ok(ctx.event.device.integration.clone().into())
                },
            },
            Function {
                name: "get_time".to_owned(),
                fun: |ctx, args| {
                    Ok(ctx.event.time.to_rfc3339().into())
                },
            },
        ]
    };

}
