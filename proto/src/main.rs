use anyhow::bail;
use pest::Parser;
use runtime::DcParser;
use crate::runtime::HatRuntime;

#[cfg(test)]
pub mod test;
mod runtime;

fn main() -> anyhow::Result<()> {
    let src = include_str!("test/sample.hat");

    let mut program = HatRuntime::default();

    program.parse("test/sample.hat".into(), src)?;

    Ok(())
}
