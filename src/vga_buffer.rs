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
	fn new(foreground: Color, background: Color) -> ColorCode
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
	buffer: &'static mut Buffer,
}

impl Writer
{
	pub fn write_byte(&mut self, byte: u8)
	{
		if byte == b'\n' || self.column_position >= BUFFER_WIDTH
		{
			self.new_line();
		}

		let row = BUFFER_HEIGHT - 1;
		let col = self.column_position;

		self.buffer.chars[row][col] = ScreenChar {
			character: byte,
			color_code: self.color_code,
		};
		self.column_position += 1;
	}

	pub fn write_string(&mut self, s: &str)
	{
		for byte in s.bytes()
		{
			match byte
			{
				0x20..0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}

	fn new_line(&mut self)
	{
		//		for row in 1..BUFFER_HEIGHT
		//		{
		//			for col in 0..BUFFER_WIDTH
		//			{
		//
		//			}
		//		}
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

pub fn write(s: &str)
{
	use core::fmt::Write;
	let mut w = Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::Green, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	};
	w.write_string(s);

	write!(w, "The numbers are {} and {}", 42, 2.0 / 3.0).unwrap();
}
