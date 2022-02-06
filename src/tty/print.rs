use core::fmt;
use crate::vga;

#[macro_export]
macro_rules! print
{
    ($($arg:tt)*) => ($crate::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println
{
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn print_to_vga()
{
	vga::Buffer::clear();
	unsafe
	{
		let len = super::TTYS[super::CURRENT_TTY].pos + super::TTYS[super::CURRENT_TTY].pos_offset;
		crate::vga_print!("{}", core::str::from_utf8(&super::TTYS[super::CURRENT_TTY].chars[0..len]).unwrap());
	}
}

pub fn print_byte_to_vga(byte: u8)
{
	unsafe
	{
		vga::W.write_byte(byte);
	}
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
	unsafe
	{
		use core::fmt::Write;
		W.write_fmt(args).unwrap();
		vga::W.write_fmt(args).unwrap();
	}
}

pub struct Writer {}

impl Writer // base stuff
{
	pub fn write_byte(&mut self, byte: u8)
	{
		super::write_byte(byte, false);
	}

	pub fn write_string(&mut self, s: &str)
	{
		for byte in s.bytes()
		{
			self.write_byte(byte);
		}
	}
}

impl fmt::Write for Writer
{
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		self.write_string(s);
		Ok(())
	}
}

pub static mut W: Writer = Writer {};
