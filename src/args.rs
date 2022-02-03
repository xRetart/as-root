use {std::process::Command, crate::error::Error};

pub fn parse() -> Result<Command, Error> {
    use std::env::args;

    let mut args = args();

    // skip argv[0] because it's the path to this executable most of the time
    args.next();

    args
        .next()
        .ok_or(Error::ParseArgs)
        .map(|name| { let mut cmd = Command::new(name); cmd.args(args); cmd })
}
