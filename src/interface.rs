pub fn issue_error(msg: &str)
{
	issue("error", msg)
}
fn issue(lvl: &str, msg: &str)
{
	const PROGRAM_NAME: &str = "as-root";
	eprintln!("[{}] {}: {}", PROGRAM_NAME, lvl, msg);
}
