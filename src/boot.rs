
pub fn ok_fail(value: bool) -> &'static str
{
	match value
	{
		true => "\x1B[32m OK \x1B[39m",
		false => "\x1B[31mFAIL\x1B[39m"
	}
}
