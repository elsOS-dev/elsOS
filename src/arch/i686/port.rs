use core::arch::asm;

#[inline(always)]
pub fn inb(port: u16) -> u8
{
	let ret: u8;
	unsafe
	{
		asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
	}
	ret
}

#[inline(always)]
pub fn inw(port: u16) -> u16
{
	let ret: u16;
	unsafe
	{
		asm!("in ax, dx", out("ax") ret, in("dx") port, options(nomem, nostack, preserves_flags));
	}
	ret
}

#[inline(always)]
pub fn ind(port: u16) -> u32
{
	let ret: u32;
	unsafe
	{
		asm!("in eax, dx", out("eax") ret, in("dx") port, options(nomem, nostack, preserves_flags));
	}
	ret
}

#[inline(always)]
pub fn outb(port: u16, value: u8)
{
	unsafe
	{
		asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
	}
}

#[inline(always)]
pub fn outw(port: u16, value: u16)
{
	unsafe
	{
		asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
	}
}

#[inline(always)]
pub fn outd(port: u16, value: u32)
{
	unsafe
	{
		asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
	}
}

#[inline(always)]
pub fn io_wait()
{
	outb(0x80, 0);
}
