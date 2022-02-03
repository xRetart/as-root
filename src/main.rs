mod error;
mod run;
mod args;

use error::Error;


fn main() {
    use std::process::exit;

    exit(code_main())
}
fn code_main() -> i32 {
    result_main()
        .unwrap_or_else(|err| { err.report(); 1 })
}
fn result_main() -> Result<i32, Error> {
    use run::run_privileged;

    args::parse()
        .and_then(run_privileged)
}
