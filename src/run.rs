use std::{ffi::OsStr, io, process::ExitStatus};

pub fn run_privileged<P, A>(program: P, args: A) -> io::Result<ExitStatus>
where
    P: AsRef<OsStr>,
    A: IntoIterator<Item = P>,
{
    use std::{process::Command, os::unix::prelude::CommandExt};
    const ROOT: u32 = 0;

    Command::new(program.as_ref())
        .args(args)
        .uid(ROOT)
        .gid(ROOT)
        .spawn()?
        .wait()
}
