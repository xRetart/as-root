use {anyhow::Result, std::process::Command};

pub fn privileged(mut cmd: Command) -> Result<Option<i32>> {
    elevate(&mut cmd);

    Ok(cmd.spawn()?.wait()?.code())
}
fn elevate(cmd: &mut Command) {
    use std::os::unix::prelude::CommandExt;

    const ROOT: u32 = 0;

    cmd.uid(ROOT).gid(ROOT);
}
