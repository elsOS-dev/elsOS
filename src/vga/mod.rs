use core::fmt;
use colors::ColorCode;
use cursor::Cursor;
use escape::Escaper;

pub mod colors;
pub mod cursor;
pub mod escape;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

pub const ESCAPE_START: u8 = 0x1B;
pub const BACKSPACE: u8 = 0x08;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar
{
	pub character: u8,
	pub color_code: ColorCode,
}

impl ScreenChar
{
	pub const fn blank() -> ScreenChar
	{
		ScreenChar
		{
			character: b' ',
			color_code: ColorCode::default(),
		}
	}
}

#[repr(transparent)]
pub struct Buffer
{
	pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer
{
	fn buffer() -> &'static mut Buffer
	{
		unsafe { &mut *(0xb8000 as *mut Buffer) }
	}

	pub fn clear_row(row: usize)
	{
		for col in 0..BUFFER_WIDTH
		{
			Buffer::buffer().chars[row][col] = ScreenChar::blank();
		}
	}

	pub fn clear()
	{
		for row in 0..BUFFER_HEIGHT
		{
			Buffer::clear_row(row);
			unsafe
			{
				if W.scroll == 0
				{
					Cursor::init(0, 15);
				}
				else
				{
					Cursor::disable();
				}
				Cursor::move_to(0, 0);
				W.col = 0;
				W.row = 0;
				W.has_overflown = false;
			}
		}
	}

	fn shift_rows(offset: usize)
	{
		for row in 0..BUFFER_HEIGHT - offset
		{
			for col in 0..BUFFER_WIDTH
			{
				Buffer::buffer().chars[row][col] = Buffer::buffer().chars[row + offset][col];
			}
		}
		for row in BUFFER_HEIGHT - offset..BUFFER_HEIGHT
		{
			Buffer::clear_row(row);
		}
	}
}

pub struct Writer
{
	cmd: Escaper,
	is_command: bool,
	color_code: ColorCode,
	pub col: usize,
	pub row: usize,
	pub scroll: usize,
	has_overflown: bool
}

impl Writer // base stuff
{
	pub fn write_byte(&mut self, byte: u8)
	{
		if self.row >= BUFFER_HEIGHT
		{
			return;
		}
		if self.is_command == true
		{
			self.escape(byte);
			return;
		}
		if self.col >= BUFFER_WIDTH && byte != b'\n'
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
				Buffer::buffer().chars[self.row][self.col] = ScreenChar
				{
					character: byte,
					color_code: self.color_code,
				};
				self.next_col();
				Cursor::move_to(self.col as u16, self.row as u16);
			},
		}
	}

	fn count_lines(s: &str) -> usize
	{
		let mut lines: usize = 0;
		let mut col: usize = 0;

		for byte in s.bytes()
		{
			if col >= BUFFER_WIDTH || byte == b'\n'
			{
				lines += 1;
				col = 0;
				continue;
			}
			col += 1;
		}
		lines
	}

	fn str_without_first_line(s: &str) -> &str
	{
		let mut len_to_remove: usize = 0;

		for byte in s.bytes()
		{
			len_to_remove += 1;
			if len_to_remove >= BUFFER_WIDTH || byte == b'\n'
			{
				break;
			}
		}
		&s[len_to_remove..]
	}

	fn str_without_last_line(s: &str) -> &str
	{
		let mut start: usize = 0;
		let mut start_candidate: usize = 0;

		for byte in s.bytes()
		{
			start += 1;
			if start - start_candidate >= BUFFER_WIDTH || byte == b'\n'
			{
				start_candidate = start - 1;
			}
		}
		&s[..start_candidate]
	}

	pub fn write_string(&mut self, s: &str)
	{
		let mut s_to_write: &str = &s[..];

		unsafe
		{
			for _ in 0..W.scroll
			{
				s_to_write = Writer::str_without_last_line(s_to_write);
			}
		}
		while Writer::count_lines(s_to_write) > BUFFER_HEIGHT
		{
			s_to_write = Writer::str_without_first_line(s_to_write);
		}

		for byte in s_to_write.bytes()
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
			Buffer::buffer().chars[row][col] = self.blank();
		}
	}

	fn new_line(&mut self)
	{
		for col in self.col..BUFFER_WIDTH
		{
			Buffer::buffer().chars[self.row][col] = ScreenChar::blank();
		}
		self.col = 0;
		self.row += 1;
		if self.row >= BUFFER_HEIGHT
		{
			self.row = BUFFER_HEIGHT - 1;
			Buffer::shift_rows(1);
			unsafe
			{
				W.has_overflown = true;
			}
		}
		self.clear_row(self.row);
		Cursor::move_to(self.col as u16, self.row as u16);
	}

	fn next_col(&mut self)
	{
		self.col += 1;
		if self.col >= BUFFER_WIDTH
		{
			self.new_line();
		}
	}

	fn prev_col(&mut self)
	{
		if self.col > 0
		{
			self.col -= 1;
		}
		else
		{
			self.col = BUFFER_WIDTH - 1;
			self.row -= 1;
		}
	}

	pub fn backspace(&mut self)
	{
		self.prev_col();
		unsafe
		{
			if cursor::CURSOR.offset == 0
			{
				Buffer::buffer().chars[self.row][self.col] = ScreenChar::blank();
			}
		}
		Cursor::move_to(self.col as u16, self.row as u16);
	}
}

impl Writer // getting static stuff
{
	fn blank(&self) -> ScreenChar
	{
		ScreenChar
		{
			character: b' ',
			color_code: self.color_code,
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
	cmd: Escaper
	{
		foreground: false,
		background: false,
		color: ColorCode::default()
	},
	is_command: false,
	color_code: ColorCode::default(),
	col: 0,
	row: 0,
	scroll: 0,
	has_overflown: false
};

pub fn scroll_up() -> bool
{
	unsafe
	{
		if W.has_overflown
		{
			W.scroll += 1;
			Cursor::disable();
			return true;
		}
	}
	false
}

pub fn scroll_down() -> bool
{
	unsafe
	{
		if W.scroll > 0
		{
			W.scroll -= 1;
			if W.scroll == 0
			{
				Cursor::init(0, 15);
			}
			return true;
		}
	}
	false
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
    unsafe
	{
    	use core::fmt::Write;
		W.write_fmt(args).unwrap();
	}
}

#[macro_export]
macro_rules! vga_print
{
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_println
{
    () => ($crate::vga_print!("\n"));
    ($($arg:tt)*) => ($crate::vga_print!("{}\n", format_args!($($arg)*)));
}
