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

pub fn get_bit_at(input: u32, n: u8) -> bool
{
	if n < 32
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

#[macro_export]
macro_rules! page_index
{
	($reg:expr) =>
	{{
		ferramenta::divide_up($reg, 4096)
	}}
}

pub fn pow(n1: usize, n2: usize) -> usize
{
	let mut r = n1;

	for _ in 1..n2
	{
		r = r * n1;
	}

	r
}

pub fn divide_up(dividend: usize, divisor: usize) -> usize
{
	(dividend + divisor - 1) / divisor
}

pub struct Bitmap
{
	pub buffer: &'static mut[u8],
	pub size: usize, // represents the number of bits, one byte = 8 bits
}

impl core::ops::Index<usize> for Bitmap
{
	type Output = bool;
	fn index<'a>(&'a self, index: usize) -> &'a bool
	{
		if self.get(index)
		{
			return &true;
		}
		else
		{
			return &false;
		}
	}
}

impl Bitmap
{
	pub fn get(&self, index: usize) -> bool
	{
		let byte_index: usize = index / 8;
		let bit_index: u8 = (index % 8) as u8;
		let bit_indexer: u8 = 0b10000000 >> bit_index;

		if self.buffer[byte_index] & bit_indexer > 0
		{
			return true;
		}
		return false;
	}

	pub fn get_chunk32(&self, index: usize) -> u32
	{
		let mut chunk: u32 = 0;

		for i in 0..4
		{
			chunk += self.buffer[index * 4 + i] as u32;
		}
		chunk
	}

	pub fn set(&mut self, index: usize, value: bool)
	{
		let byte_index: usize = index / 8;
		let bit_index: u8 = (index % 8) as u8;
		let bit_indexer: u8 = 0b10000000 >> bit_index;

		self.buffer[byte_index] &= !bit_indexer;
		if value
		{
			self.buffer[byte_index] |= bit_indexer;
		}
	}
	pub fn debug_print(&self, len: usize)
	{
		if len == 0
		{
			for i in 0..self.size
			{
				crate::logln!("{}", self.get(i));
			}
		}
		else
		{
			for i in 0..len
			{
				crate::logln!("{}", self.get(i));
			}
		}
	}
	pub fn erase(&mut self)
	{
		for i in 0..self.size
		{
			self.set(i, true);
		}
	}
}

pub fn set_bit(var: &mut u32, value: bool, bit: u8)
{
	if value
	{
		*var |= (value as u32) << bit;
	}
	else
	{
		*var &= !(!value as u32) << bit;
	}
}

pub fn align(val: usize, bound: usize) -> usize
{
	val + bound - 1 & !(bound - 1)
}

pub fn in_range(input: usize, start: usize, range: usize) -> bool
{
	input >= start && input < start + range
}
