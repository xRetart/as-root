use {
    anyhow::{anyhow, Error, Result},
    std::{path::Path, str::FromStr},
    users::uid_t,
};

pub struct Data {
    pub permitted_users: Vec<uid_t>,
}
impl Data {
    pub fn from_file(path: &Path) -> Result<Self> {
        use std::fs::read_to_string;

        read_to_string(path)
            .map_err(|_| anyhow!("Configuration not found."))
            .and_then(|contents| contents.parse())
    }
}
impl FromStr for Data {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn username_to_id(name: &str) -> Result<uid_t, Error> {
            use users::get_user_by_name;

            get_user_by_name(name)
                .map(|name| name.uid())
                .ok_or_else(|| anyhow!("Configuration contains invalid user."))
        }

        s.lines()
            .map(|line| username_to_id(line.trim()))
            .collect::<Result<_, _>>()
            .map(|permitted_users| Self { permitted_users })
    }
}
