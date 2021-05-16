pub fn issue_error(msg: &str)
{
	issue("error", msg)
}
#[cfg(debug_assertions)]
pub fn issue_debug(msg: &str)
{
	issue("debug", msg)
}
fn issue(lvl: &str, msg: &str)
{
	const PROGRAM_NAME: &str = "as-root";

	eprintln!("[{}] {}: {}", PROGRAM_NAME, lvl, msg);
}
