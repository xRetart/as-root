use crate::error;

pub fn run(cmd: Vec<String>) -> error::EnumResult<std::process::ExitStatus>
{
	assert_privileges()?;
	elevate();

	let (program, args) = cmd.split_at(1);
	execute(
		program
			.first()
			.unwrap()
			.as_str(),

		args
			.iter()
			.map(|arg| arg.as_ref())
			.collect::<Vec<&str>>()
	)
		.map_err(|err| error::Error::System(err.to_string()))
}
fn assert_privileges() -> error::EnumResult<()>
{
	if is_privileged()?
	{
		Ok(())
	}
	else
	{
		Err(crate::error::Error::Unprivileged)
	}
}
fn is_privileged() -> error::EnumResult<bool>
{
	Ok(
		get_privileged_users()
			.ok_or(error::Error::FileInaccessible)?
			.contains(&unsafe { libc::getuid() })
	)
}
fn get_privileged_users() -> Option<Vec<libc::uid_t>>
{
	use std::convert::TryInto;
	const CONFIG_PATH: &str = "/etc/as-root.conf";

	Some(
		std::fs::read(CONFIG_PATH)
			.ok()?
			.windows(4)
			.map(|bytes| u32::from_ne_bytes(bytes.try_into().expect("invalid conf")))
			.collect::<Vec<libc::uid_t>>()
	)
}
fn elevate()
{
	const ROOT: libc::uid_t = 0;
	unsafe
	{
		libc::setuid(ROOT);
		libc::setgid(ROOT);
	}
}
fn execute(path: &str, args: Vec<&str>) -> std::io::Result<std::process::ExitStatus>
{
	std::process::Command::new(path)
		.args(args)
		.status()
}
