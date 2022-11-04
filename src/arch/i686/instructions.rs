use core::arch::asm;
use super::interrupts::idt;

#[inline(always)]
pub unsafe fn lidt(idtr: &idt::descriptor)
{
	asm!("lidt [{}]", in(reg) idtr, options(nostack));
}

#[inline(always)]
pub unsafe fn sti()
{
	asm!("sti");
}

#[inline(always)]
pub unsafe fn cli()
{
	asm!("cli");
}
