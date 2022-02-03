use {std::process::Command, crate::error::Error};

pub fn run_privileged(mut cmd: Command) -> Result<i32, Error> {
    use std::os::unix::prelude::CommandExt;

    const ROOT: u32 = 0;

    cmd
        .uid(ROOT)
        .gid(ROOT)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(Error::Io)
        .and_then(|status| status.code().ok_or(Error::NoExitCode))
}
