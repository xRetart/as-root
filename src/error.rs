use std::io;


#[derive(Debug)]
pub enum Error {
    NoArgs,
    NoExitCode,
    UserDenied,
    MissingConfig,
    ConfigInvalidUser,
    Io(io::Error),
}
impl Error {
    pub fn report(&self) {
        const PROJECT_NAME: &str = "as-root";

        eprintln!("{} [ERROR]: {}", PROJECT_NAME, self.to_string());
    }
}
impl ToString for Error {
    fn to_string(&self) -> String {
        use Error::*;

        match self {
            NoArgs => "No arguments given.".to_owned(),
            NoExitCode => "Command did not set exit code.".to_owned(),
            UserDenied => "Current user not permitted to run as-root".to_owned(),
            MissingConfig => "Configuration file /etc/conf.d/as-root is missing.".to_owned(),
            ConfigInvalidUser => "Invalid user in configuration.".to_owned(),
            Io(err) => err.to_string(),
        }
    }
}
