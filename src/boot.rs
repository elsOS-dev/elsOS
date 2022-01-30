
pub fn ok_fail(value: bool) -> &'static str
{
	match value
	{
		true => "\x1B32m OK \x1B38m",
		false => "\x1B31mFAIL\x1B38m"
	}
}
