use anyhow::Context;
use crate::runtime::HatRuntime;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod runtime;
#[cfg(test)]
pub mod test;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    
    let src = include_str!("test/sample.hat");

    let mut program = HatRuntime::new("wss://ha.polaris.fleap.dev/api/websocket").await
        .context("failed to initialize runtime")?;

    // program.parse("test/sample.hat".into(), src)?;

    Ok(())
}
