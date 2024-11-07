#![feature(async_drop)]

use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
};

use crate::integrations::dummy::DummyIntegration;
use crate::runtime::HatRuntime;
use anyhow::Context;
use clap::Parser;
use integrations::home_assistant::HassIntegration;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub mod integrations;
pub mod runtime;
pub mod server;
#[cfg(test)]
pub mod test;

#[derive(clap::Parser, Debug)]
#[command(
    version,
    about = "Hat CLI to work with Hat automation files",
    long_about = None,
    author = "Felipe Paixão <fleap@fleap.dev>",
)]
struct CliArguments {
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "run as a server for the visual dsl frontend"
    )]
    serve: bool,

    #[arg(
        short,
        long,
        default_value_t = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 5000)),
        help = "address of the HTTP server"
    )]
    address: SocketAddr,

    #[arg(help = "the HAT source file")]
    file: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    read_env_files();

    let args = CliArguments::parse();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let path_string = args
        .file
        .to_str()
        .unwrap_or("PATH CONTAINS INVALID UNICODE")
        .to_owned();

    let source = std::fs::read_to_string(&args.file)
        .with_context(|| format!("failed to read file: {:?}", args.file))?;

    let runtime = HatRuntime::new().await;

    runtime.parse(path_string, &source)?;

    runtime.integrate(DummyIntegration::new()).await;
    runtime.integrate(HassIntegration::new(
        "https://ha.polaris.fleap.dev",
       "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiI3YzhiYjdkMDczYmY0OWFiYTc4YTY0YjVmMzZkYTkwNiIsImlhdCI6MTcyMjQzNzk3NywiZXhwIjoyMDM3Nzk3OTc3fQ.h8uzazAaV_4MopUB3vPu258l54bhoh4DuZc30shF42M"
    ).await?).await;

    let router = server::make_router(runtime);

    let listener = tokio::net::TcpListener::bind(&args.address)
        .await
        .with_context(|| format!("failed to bind address: {}", args.address))?;

    info!("Starting HTTP server at {}", args.address);

    axum::serve(listener, router)
        .await
        .context("failed to execute http server")?;

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
