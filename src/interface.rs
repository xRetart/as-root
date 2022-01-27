use std::env::{args, Args};

pub struct Command {
    pub program: String,
    pub args: Args,
}
pub fn parse_args() -> Option<Command> {
    let args = args();

    args
        .next()
        .map(|program| Command { program, args })
}
