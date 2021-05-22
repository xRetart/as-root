use crate::error;


pub fn configure(config: &str) -> error::EnumResult<()>
{
	const OUTPUT_FILENAME: &str = "/etc/as-root.conf";

	process_config(
		&(std::fs::File::open(config)).map_err(|err| error::Error::System(err.to_string()))?,
		&mut (std::fs::File::create(OUTPUT_FILENAME).map_err(|err| error::Error::System(err.to_string())))?,
	)
}
fn process_config(input: &std::fs::File, output: &mut std::fs::File) -> error::EnumResult<()>
{
	parse_config(&input).write(output)
}
fn parse_config(file: &std::fs::File) -> Config
{
	Config
	{
		users:
			get_lines(file)
			.map(|result| result.expect("failed lines"))
			.collect::<Vec<String>>(),
	}
}

type Lines<'a> = std::io::Lines<std::io::BufReader<&'a std::fs::File>>;
fn get_lines(file: &std::fs::File) -> Lines
{
	use std::io::BufRead;
	std::io::BufReader::new(file).lines()
}

struct Config
{
	users: Vec<String>,
}

impl Config
{
	pub fn write(self, file: &mut std::fs::File) -> error::EnumResult<()>
	{
		for name in self.users
		{
			write_uid(
				file,
				get_uid(name.clone()).ok_or(error::Error::UserInvalid)?
			)?;
		}
		Ok(())
	}
}
fn get_uid(name: String) -> Option<libc::uid_t>
{
	let c_name = std::ffi::CString::new(name).unwrap();
	unsafe
	{
		let user = libc::getpwnam(c_name.as_ptr());
		if user.is_null()
		{
			None
		}
		else
		{
			Some((*user).pw_uid)
		}
	}
}
fn write_uid(file: &mut std::fs::File, uid: libc::uid_t) -> error::EnumResult<()>
{
	use std::io::Write;

	file
		.write_all(&uid.to_ne_bytes())
		.map_err(|err| error::Error::System(err.to_string()))
}
