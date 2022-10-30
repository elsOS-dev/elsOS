use core::arch::asm;

pub mod gdt;
pub mod instructions;
pub mod interrupts;
pub mod port;

pub fn init()
{
	gdt::init();
}

#[inline(always)]
pub fn halt()
{
	unsafe
	{
		asm!("hlt");
	}
}
