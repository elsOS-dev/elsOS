use core::fmt;
use crate::vga::cursor::move_cursor;
use crate::vga::colors::ColorCode;
use crate::vga::colors::Color;
use crate::vga::escape::Escaper;

mod escape;
mod colors;
pub mod cursor;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

const ESCAPE_START: u8 = 0x1B;
const BACKSPACE: u8 = 0x08;

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
	cmd: Escaper,
	is_command: bool,
	column_position: usize,
	color_code: ColorCode,
}

impl Writer // base stuff
{
	pub fn write_byte(&mut self, byte: u8)
	{
		if self.is_command == true
		{
			self.escape(byte);
			return;
		}
		let row = BUFFER_HEIGHT - 1;

		if self.column_position >= BUFFER_WIDTH && byte != b'\n'
		{
			self.new_line();
		}
		match byte
		{
			b'\n' => self.new_line(),
			BACKSPACE  => self.backspace(),
			ESCAPE_START  => self.is_command = true,
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
				0x00..0xfd => self.write_byte(byte),
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
}

impl Writer // getting static stuff
{
	fn buffer() -> &'static mut Buffer
	{
		unsafe { &mut *(0xb8000 as *mut Buffer) }
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

const DEFAULT_COLOR: ColorCode = ColorCode::new(Color::White, Color::Blue);

static mut W: Writer = Writer
{
cmd: Escaper {foreground: false, background: false, color: DEFAULT_COLOR},
	is_command: false,
	column_position: 0,
	color_code: DEFAULT_COLOR,
};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
    use core::fmt::Write;

	unsafe { W.write_fmt(args).unwrap(); }
}

