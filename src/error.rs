use std::io;


#[derive(Debug)]
pub enum Error {
    ParseArgs,
    NoExitCode,
    Io(io::Error),
}
impl Error {
    pub fn report(&self) {
        use Error::*;

        let message = match self {
            ParseArgs => "Failed to parse given arguments.".to_owned(),
            NoExitCode => "Command did not set exit code.".to_owned(),
            Io(err) => err.to_string(),
        };
        let project_name = "as-root";

        eprintln!("{} [ERROR]: {}", project_name, message);
    }
}
