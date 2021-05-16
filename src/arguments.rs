pub fn get_action() -> Result<Action, &'static str>
{
	get_user_args()
		.map(|args| parse_args(args))
		.ok_or("Unrecognized OS argument supply (no argv[0])")
}
pub enum Action
{
	Command(Vec<String>),
	Configuration(String),
	Error(&'static str),
}
fn get_user_args() -> Option<Vec<String>>
{
	let mut args = std::env::args();
	args.next().map(|_| args.collect::<Vec<String>>())
}
fn parse_args(args: Vec<String>) -> Action
{
	if !args.is_empty()
	{
		Action::Command(args)
	}
	else
	{
		Action::Error("No command given.")
	}
}
