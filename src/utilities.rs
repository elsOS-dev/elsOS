use core::arch::asm;
use core::slice;

pub fn shutdown_qemu()
{
	outw(0x604, 0x2000);
}

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

pub unsafe fn strlen(str: *const u8) -> usize
{
	let mut i = 0;

	while *str.add(i) != 0
	{
		i += 1;
	}

	i
}

pub unsafe fn from_c_str(ptr: *const u8) -> &'static [u8]
{
	slice::from_raw_parts(ptr, strlen(ptr) + 1)
}

pub unsafe fn print_memory(ptr: *const u8, n: usize)
{
	let mut i: usize = 0;

	while i < n
	{
		if i % 16 == 0
		{
			crate::log!("{:08x}: ", ptr.add(i) as u32);
		}
		crate::log!("{:02x?} ", *ptr.add(i));
		i += 1;
		if i % 16 == 0
		{
			crate::log!(" |");
			for i in i - 16..i
			{
				let chr = *ptr.add(i);
				crate::log!("{}", if chr > 0x1f && chr < 0x7f {chr as char } else { '.' });
			}
			crate::log!("|");
			crate::logln!();
		}
		else if i % 8 == 0
		{
			crate::log!("  ");
		}
	}
	crate::logln!();
}

pub unsafe fn print_memory_bin(ptr: *const u8, n: usize)
{
	let mut i: usize = 0;

	while i < n
	{
		if i % 4 == 0
		{
			crate::log!("{:08x}:     ", ptr.add(i) as u32);
		}
		crate::log!("{:08b}   ", *ptr.add(i));
		i += 1;
		if i % 4 == 0
		{
			crate::logln!();
		}
	}
	crate::logln!();
}

#[macro_export]
macro_rules! get_reg
{
	($reg:expr) =>
	{{
		let val: u32;
		core::arch::asm!(concat!("mov {}, ", $reg), out(reg) val);
		val
	}}
}
