pub fn get_bit_at(input: u8, n: u8) -> bool
{
	if n < 8
	{
        return input & (1 << n) != 0;
	}
	false
}

