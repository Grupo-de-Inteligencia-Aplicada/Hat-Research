use crate::runtime::HatRuntime;
use anyhow::Context;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub mod home_assistant;
mod runtime;
#[cfg(test)]
pub mod test;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    read_env_files();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let src = include_str!("test/sample.hat");

    let mut runtime = HatRuntime::new("wss://ha.polaris.fleap.dev/api/websocket", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiI3YzhiYjdkMDczYmY0OWFiYTc4YTY0YjVmMzZkYTkwNiIsImlhdCI6MTcyMjQzNzk3NywiZXhwIjoyMDM3Nzk3OTc3fQ.h8uzazAaV_4MopUB3vPu258l54bhoh4DuZc30shF42M").await
        .context("failed to initialize runtime")?;

    runtime.parse("test/sample.hat".into(), src)?;

    Ok(())
}

fn read_env_files() {
    let mut env: &str;
    let env_var = std::env::var("RUST_ENV").unwrap_or("".into());
    env = &env_var;
    if env.is_empty() {
        env = "development";
    }

    dotenvy::from_filename(format!(".env.{env}.local")).ok();
    if "test" != env {
        dotenvy::from_filename(".env.local").ok();
    }
    dotenvy::from_filename(format!(".env.{env}")).ok();
    dotenvy::dotenv().ok();
}
