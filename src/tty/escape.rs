use crate::tty::Writer;
use crate::vga::colors::Color;
use crate::vga::colors::ColorCode;

pub struct Escaper
{
	pub foreground: bool, // first number is 3
	pub background: bool, // first number is 4
	pub color: ColorCode,
}

impl Writer //color escape commands
{
	fn default_color(&self) -> Color
	{
		if self.cmd.foreground
		{
			Color::White
		}
		else
		{
			Color::Blue
		}
	}

	fn colors(&mut self, byte: u8)
	{
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
		else
		{
			self.color_code = self.cmd.color;
		}
	}
}

impl Writer // Escaper related stuff
{
	fn escape_off(&mut self)
	{
		self.cmd.foreground = false;
		self.cmd.background = false;
	}

	fn cmd_off(&mut self)
	{
		self.is_command = false;
		self.escape_off();
	}

	pub fn escape(&mut self, byte: u8)
	{
		match byte
		{
			b'a'..b'z' | b'A'..b'Z' => self.cmd_off(),
			b';' => { self.escape_off(); return; },
			_ => {},
		}
		self.colors(byte);
	}
}
