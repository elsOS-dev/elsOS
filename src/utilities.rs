use core::arch::asm;

#[inline(always)]
pub fn outw(port: u32, value: u16)
{
    unsafe
    {
        asm!("out dx, ax", in("dx") port, in("ax") value,
                options(nomem, nostack, preserves_flags));
    }
}

#[inline(always)]
pub fn inb(port: u32) -> u8
{
    let ret: u8;
    unsafe
    {
        asm!("in al, dx", out("al") ret, in("dx") port,
                options(nomem, nostack, preserves_flags));
    }
    ret
}

#[inline(always)]
pub fn outb(port: u32, value: u8)
{
    unsafe
    {
        asm!("out dx, al", in("dx") port, in("al") value,
                options(nomem, nostack, preserves_flags));
    }
}

pub fn get_bit_at(input: u8, n: u8) -> bool
{
	if n < 8
	{
        return input & (1 << n) != 0;
	}
	false
}

