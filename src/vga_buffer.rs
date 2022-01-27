use crate::utilities::{inb, outb};
use crate::keyboard::Arrow;
use core::fmt;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color
{
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode
{
	const fn new(foreground: Color, background: Color) -> ColorCode
	{
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar
{
	character: u8,
	color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer
{
	chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer
{
	column_position: usize,
	color_code: ColorCode,
}

impl Writer
{
	pub fn write_byte(&mut self, byte: u8)
	{
		let row = BUFFER_HEIGHT - 1;

		if self.column_position >= BUFFER_WIDTH && byte != b'\n'
		{
			self.new_line();
		}
		match byte
		{
			b'\n' => self.new_line(),
			0x08  => self.backspace(),
			_ =>
			{
				let col = self.column_position;

				Writer::buffer().chars[row][col] = ScreenChar {
					character: byte,
					color_code: self.color_code,
				};
				self.column_position += 1;
			},
		}
		move_cursor(self.column_position as u16, row as u16);
	}

	pub fn write_string(&mut self, s: &str)
	{
		for byte in s.bytes()
		{
			match byte
			{
				0x20..0x7e | b'\n' | 0x08 => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}
	fn clear_row(&mut self, row: usize)
	{
		for col in 0..BUFFER_WIDTH
		{
			Writer::buffer().chars[row][col] = self.blank();
		}
	}

	fn new_line(&mut self)
	{
		for row in 1..BUFFER_HEIGHT
		{
			for col in 0..BUFFER_WIDTH
			{
				let character = Writer::buffer().chars[row][col];
				Writer::buffer().chars[row - 1][col] = character;
			}
		}
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	fn backspace(&mut self)
	{
		if self.column_position > 0
		{
			self.column_position -= 1;
			Writer::buffer().chars[BUFFER_HEIGHT - 1][self.column_position] = self.blank();
		}
	}

	fn blank(&self) -> ScreenChar
	{
		let blank = ScreenChar
		{
			character: b' ',
			color_code: self.color_code,
		};
		blank
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

impl Writer
{
	fn buffer() -> &'static mut Buffer
	{
		unsafe { &mut *(0xb8000 as *mut Buffer) }
	}
}

#[macro_export]
macro_rules! print
{
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println
{
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


static mut W: Writer = Writer
{
	column_position: 0,
	color_code: ColorCode::new(Color::White, Color::Blue),
};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
    use core::fmt::Write;

	unsafe { W.write_fmt(args).unwrap(); }
}

const CRT_ADDR_REG: u32 = 0x3D4;
const CRT_DATA_REG: u32 = 0x3D5;

const CURSOR_START_REG: u8 = 0x0A;
const CURSOR_END_REG: u8 = 0x0B;

const CURSOR_HIGH_REG: u8 = 0x0E;
const CURSOR_LOW_REG: u8 = 0x0F;

pub fn init_cursor(cursor_start: u8, cursor_end: u8)
{
	outb(CRT_ADDR_REG, CURSOR_START_REG);
	outb(CRT_DATA_REG, (inb(CRT_DATA_REG) & 0xC0) | cursor_start);

	outb(CRT_ADDR_REG, CURSOR_END_REG);
	outb(CRT_DATA_REG, (inb(CRT_DATA_REG) & 0xE0) | cursor_end);
}

fn move_cursor(x: u16, y: u16)
{
	let pos: u16 = y * BUFFER_WIDTH as u16 + x;

	outb(CRT_ADDR_REG, CURSOR_LOW_REG);
	outb(CRT_DATA_REG, pos as u8 & 0xFF);
	outb(CRT_ADDR_REG, CURSOR_HIGH_REG);
	outb(CRT_DATA_REG, (pos >> 8) as u8 & 0xFF);
}

pub fn handle_arrows(arrow: Arrow)
{
	let row = BUFFER_HEIGHT - 1;

	unsafe
	{
		match arrow
		{
			Arrow::Left => W.column_position -= 1,
			Arrow::Right => W.column_position += 1,
		};
		move_cursor(W.column_position as u16, row as u16);
	}
}
