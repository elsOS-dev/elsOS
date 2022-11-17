use core::arch::asm;

pub fn rand() -> u32
{
	unsafe
	{
		let random_value: u32;

		asm!("rdtsc");
		asm!("mov {}, eax", out(reg) random_value);
		random_value
	}
}
