extern crate libc;


pub type RunResult = std::io::Result<std::process::ExitStatus>;
pub fn run(cmd: Vec<String>) -> RunResult
{
	elevate_permission();
	execute(
		cmd[0].as_str(),
		cmd[1..]
			.iter()
			.map(|arg| arg.as_ref())
			.collect::<Vec<&str>>()
	)
}
fn elevate_permission()
{
	const ROOT: libc::uid_t = 0;
	unsafe
	{
		libc::setuid(ROOT);
		libc::setgid(ROOT);
	}
}
fn execute(path: &str, args: Vec<&str>) -> RunResult
{
	std::process::Command::new(path).args(args).status()
}

// TODO
pub fn configure(config: String)
{
	println!("to configure: \"{}\"", config);
}