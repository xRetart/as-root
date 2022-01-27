#![feature(termination_trait_lib)]

mod interface;
mod log;
mod run;

use std::process::ExitCode;

// top most level main function used to safely exit with code
fn main() -> ExitCode {
    use std::process::exit;

    exit(status_main());
}

// 2nd most top level main evaluating all errors
fn status_main() -> i32 {
    use {run::run_privileged, interface::parse_args};

    let command = parse_args();

    match run_privileged(command.program, command.args) {
        Ok(status) => status.code().unwrap_or(0),
        Err(_) => { log::error("failed to run command"); 1 },
    }
}

// lowest level main function doing all actual processing and returning an error
fn real_main() -> Option<MainError> {
}
