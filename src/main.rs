mod run;
mod args;
mod config;

use anyhow::{anyhow, Result};


fn main() {
    use std::process::exit;

    if let Some(code) = code_main() {
        exit(code);
    }
}
fn code_main() -> Option<i32> {
    result_main()
        .unwrap_or_else(|err| { eprintln!("{}", err); Some(1) })
}
fn result_main() -> Result<Option<i32>> {
    use {std::path::Path, users::get_current_uid};

    let config = config::Data::from_file(Path::new("/etc/conf.d/as-root"))?;

    if config.permitted_users.contains(&get_current_uid()) {
        args::parse()
            .and_then(run::privileged)
    }
    else {
        Err(anyhow!("Permission denied."))
    }
}
