use {std::process::Command, anyhow::{anyhow, Result}};

pub fn parse() -> Result<Command> {
    use std::env::args;

    let mut args = args();

    // skip argv[0] because it's the path to this executable most of the time
    args.next();

    let name = args.next().ok_or_else(|| anyhow!("Not enough arguments."))?;
    let mut cmd = Command::new(name);
    cmd.args(args);

    Ok(cmd)
}
