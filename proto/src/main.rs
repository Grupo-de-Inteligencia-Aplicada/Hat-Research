use crate::runtime::HatRuntime;

mod runtime;
#[cfg(test)]
pub mod test;

fn main() -> anyhow::Result<()> {
    let src = include_str!("test/sample.hat");

    let mut program = HatRuntime::default();

    program.parse("test/sample.hat".into(), src)?;

    Ok(())
}
