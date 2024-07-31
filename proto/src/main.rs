use anyhow::Context;
use crate::runtime::HatRuntime;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod runtime;
#[cfg(test)]
pub mod test;
pub mod home_assistant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let src = include_str!("test/sample.hat");

    let mut program = HatRuntime::new("wss://ha.polaris.fleap.dev/api/websocket", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiI3YzhiYjdkMDczYmY0OWFiYTc4YTY0YjVmMzZkYTkwNiIsImlhdCI6MTcyMjQzNzk3NywiZXhwIjoyMDM3Nzk3OTc3fQ.h8uzazAaV_4MopUB3vPu258l54bhoh4DuZc30shF42M").await
        .context("failed to initialize runtime")?;

    // program.parse("test/sample.hat".into(), src)?;

    Ok(())
}
