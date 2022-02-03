mod error;
mod run;
mod args;
mod config;

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
    use {std::path::Path, users::get_current_uid};
    
    let config = config::Data::from_file(Path::new("/etc/conf.d/as-root"))?;
    
    if config.permitted_users.contains(&get_current_uid()) {
        args::parse()
            .and_then(run::privileged)
    }
    else {
        Err(Error::UserDenied)
    }
}
