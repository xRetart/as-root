mod arguments;
mod interface;
mod app;


fn main()
{
	use arguments::Action;

	match arguments::get_action()
	{
		// usual cases
		Ok(Action::Command(cmd)) => run_command(cmd),
		Ok(Action::Configuration(config)) => app::configure(config),

		// error cases
		Ok(Action::Error(msg)) |
		Err(msg) => interface::issue_error(msg),
	}
}
#[allow(unused_variables)]
fn run_command(cmd: Vec<String>)
{
	let result = app::run(cmd);
	
	#[cfg(debug_assertions)]
	if result.is_err()
	{
		interface::issue_debug("Execution failed.")
	}
}
