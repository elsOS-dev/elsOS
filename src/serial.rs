use core::fmt;
use crate::arch::port::{inb, outb};

#[macro_export]
macro_rules! serial_print
{
	($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println
{
	() => ($crate::serial_print!("\n"));
	($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

pub const COM1: u16 = 0x3f8;

fn check_serial_chip(port: u16) -> bool
{
	outb(port, 0xAE);
	inb(port) == 0xAE
}

pub fn init(port: u16) -> bool
{
	unsafe
	{
		if !crate::SETTINGS.has_serial
		{
			return false;
		}
	}
	outb(port + 1, 0x00); // Disable all interrupts
	outb(port + 3, 0x80); // Enable DLAB (set baud rate divisor)
	outb(port + 0, 0x03); // Set divisor to 3 (lo byte) (115 200 / 3 => 38400 baud)
	outb(port + 1, 0x00); //                  (hi byte)
	outb(port + 3, 0x03); // 8 bits, no parity, one stop bit
	outb(port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
	outb(port + 4, 0x0B); // IRQs enabled, RTS/DSR set
	outb(port + 4, 0x1E); // Set in loopback mode, test the serial chip

	if !check_serial_chip(port)
	{
		return false;
	}

	// serial is not faulty, set to normal operation mode
	outb(port + 4, 0x0F);
	return true;
}

fn is_transmit_empty(port: u16) -> u8 {
	return inb(port + 5) & 0x20;
}

pub fn write(a: u8, port: u16) {
	while is_transmit_empty(port) == 0 { }
	outb(port, a);
}

struct Serial
{
	port: u16,
	pos_x: usize
}

static mut SERIAL: Serial = Serial
{
	port: COM1,
	pos_x: 0
};

pub struct Writer
{

}

impl Writer // base stuff
{
	pub fn write_byte(&mut self, byte: u8)
	{
		unsafe
		{
			match byte
			{
				b'\n' =>
				{
					self.new_line();
				},
				_ =>
				{
					write(byte, SERIAL.port);
					SERIAL.pos_x += 1;
				},
			}
		}
	}

	pub fn write_string(&mut self, s: &str)
	{
		for byte in s.bytes()
		{
			match byte
			{
				0x00..0xfd => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}

	fn new_line(&mut self)
	{
		unsafe
		{
			write(b'\n', SERIAL.port);
			SERIAL.pos_x = 0;
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

pub static mut W: Writer = Writer
{

};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
	unsafe
	{
		if !crate::SETTINGS.has_serial
		{
			return;
		}
		else
		{
			use core::fmt::Write;
			W.write_fmt(args).unwrap();
		}
	}
}
