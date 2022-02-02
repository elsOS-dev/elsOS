use colors::ColorCode;

pub mod colors;
pub mod cursor;

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
struct Buffer
{
	chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
