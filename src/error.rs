#[derive(strum_macros::AsStaticStr)]
pub enum Error
{
	Unprivileged,
	FileInaccessible,
	UserInvalid,
	System(String),
}

pub type EnumResult<T> = Result<T, Error>;
