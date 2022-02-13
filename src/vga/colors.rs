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
pub struct ColorCode(u8);

impl ColorCode
{
	pub const fn new(foreground: Color, background: Color) -> ColorCode
	{
		ColorCode((background as u8) << 4 | (foreground as u8))
	}

	pub const fn new_i(foreground: u8, background: u8) -> ColorCode
	{
		ColorCode((background << 4) | foreground)
	}

	pub fn fg(self) -> u8
	{
		self.0 & 0xf
	}

	pub fn bg(self) -> u8
	{
		self.0 >> 4
	}

	pub const fn default() -> ColorCode
	{
		ColorCode::new(Color::White, Color::Black)
	}

	pub const fn panic() -> ColorCode
	{
		ColorCode::new(Color::White, Color::Red)
	}
}
