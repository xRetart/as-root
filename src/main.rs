mod arguments;
mod interface;
mod actions;
mod error;


fn main()
{
	use arguments::Action;

	match arguments::get_action()
	{
		// usual cases
		Ok(Action::Command(cmd)) => run(cmd),
		Ok(Action::Configuration(config)) => configure(&config),

		// error case
		Err(msg) => interface::issue_error(msg),
	}
}

fn run(cmd: Vec<String>)
{
	handle_result(&actions::elevate::run(cmd))
}
fn configure(config: &String)
{
	handle_result(&actions::configure::configure(config.as_str()))
}


fn handle_result<T>(result: &error::EnumResult<T>)
{
	if let Err(err) = result { handle_error(err); }
}
fn handle_error(err: &error::Error)
{
	if let error::Error::System(msg) = err
	{
		interface::issue_error(msg);
	}
	else
	{
		use strum::AsStaticRef;
		interface::issue_error(err.as_static())
	}
}
