use crate::vga::Writer;
use crate::vga::Color;
use crate::vga::ColorCode;

pub struct Escaper
{
	pub foreground: bool, // first number is 3
	pub background: bool, // first number is 4
	pub color: ColorCode,
}

impl Writer // Escaper related stuff
{
	pub fn escape(&mut self, byte: u8)
	{
		if (byte >= 0x41 && byte <= 0x5a) || (byte >= 0x61 && byte <= 0x7a)
		{
			self.is_command = false;
			self.cmd.foreground = false;
			self.cmd.background = false;
			return;
		}
		if byte == b';'
		{
			self.cmd.foreground = false;
			self.cmd.background = false;
			return ;
		}
		if self.cmd.foreground ^ self.cmd.background
		{
			let color = match byte
			{
				b'0' => Color::Black,
				b'1' => Color::Red,
				b'2' => Color::Green,
				b'3' => Color::Brown,
				b'4' => Color::Blue,
				b'5' => Color::Magenta,
				b'6' => Color::Cyan,
				b'7' => Color::White,
				_ => self.default_color(),
			};
			if self.cmd.foreground
			{
				self.cmd.color = self.color_code;
				self.color_code = ColorCode::new_i(color as u8, self.color_code.bg());
				self.cmd.background = true;
			}
			else if self.cmd.background
			{
				self.cmd.color = self.color_code;
				self.color_code = ColorCode::new_i(self.color_code.fg(), color as u8);
				self.cmd.foreground = true;
			}
		}
		else if !self.cmd.foreground && !self.cmd.background
		{
			match byte
			{
				b'3' => self.cmd.foreground = true,
				b'4' => self.cmd.background = true,
				_ => {},
			}
		}
	}

	fn default_color(&self) -> Color
	{
		if self.cmd.foreground
		{
			return Color::White;
		}
		else
		{
			return Color::Blue;
		}
	}
}

