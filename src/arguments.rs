pub fn get_action() -> Result<Action, &'static str>
{
	get_user_args()
		.map(parse_args)
		.ok_or("Unrecognized OS argument supply (no argv[0])")
}
pub enum Action
{
	Command(Vec<String>),
	Configuration(String),
}
fn get_user_args() -> Option<Vec<String>>
{
	let mut args = std::env::args();
	args.next()?;

	Some(args.collect::<Vec<String>>())
}

fn parse_args(args: Vec<String>) -> Action
{
	const CONFIGURE_FLAG: &str = "-c";
	
	if contains(&args.first().map(String::as_str), CONFIGURE_FLAG)
	{
		Action::Configuration(args[1].clone())
	}
	else
	{
		Action::Command(args)
	}
}


fn contains<T>(op: &Option<T>, val: T) -> bool
where
	T: std::cmp::PartialEq
{
	op.is_some() && *op.as_ref().unwrap() == val
}
