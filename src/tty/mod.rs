use core::fmt;
use crate::keyboard;
use crate::vga;

use escape::Escaper;

use crate::vga::colors::ColorCode;

use crate::utilities::shutdown_qemu;

pub mod escape;

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

const BUFFER_HEIGHT: usize = 2 * vga::BUFFER_HEIGHT;
const BUFFER_WIDTH: usize = vga::BUFFER_WIDTH;

static mut VGA: *mut Tty = 0xb8000 as *mut Tty;

#[repr(C)]
struct Tty
{
	chars: [[vga::ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
	pos_x: usize,
	pos_y: usize,
	has_overflown: bool
}

impl Tty
{
	fn print_to_vga(&self)
	{
		unsafe
		{
			for row in 0..vga::BUFFER_HEIGHT
			{
				for col in 0..vga::BUFFER_WIDTH
				{
					let src_row = match self.has_overflown
					{
						true => (self.pos_y as isize - vga::BUFFER_HEIGHT as isize + row as isize + 1).rem_euclid(BUFFER_HEIGHT as isize) as usize,
						false => if self.pos_y >= vga::BUFFER_HEIGHT
							{
								row + self.pos_y - vga::BUFFER_HEIGHT + 1
							}
							else
							{
								row
							}
					};
					(*VGA).chars[row][col] = self.chars[src_row % BUFFER_HEIGHT][col]
				}
			}
			self.move_cursor();
		}
	}

	fn move_cursor(&self)
	{
		let cursor_row = match self.has_overflown
		{
			true => vga::BUFFER_HEIGHT - 1,
			false => if self.pos_y >= vga::BUFFER_HEIGHT
				{
					vga::BUFFER_HEIGHT - 1
				}
				else
				{
					self.pos_y
				}
		};
		vga::cursor::move_to(self.pos_x as u16, cursor_row as u16);
	}
}

static mut CURRENT_TTY: usize = 0;
static mut TTYS: [Tty; 1] =
[
	Tty
	{
		chars: [[vga::ScreenChar::blank(); BUFFER_WIDTH]; BUFFER_HEIGHT],
		pos_x: 0,
		pos_y: 0,
		has_overflown: false
	}
];

pub struct Writer
{
	cmd: Escaper,
	is_command: bool,
	color_code: ColorCode,
}

impl Writer // base stuff
{
	pub fn write_byte(&mut self, byte: u8)
	{
		unsafe
		{
			if self.is_command == true
			{
				self.escape(byte);
				return;
			}
			let row = TTYS[CURRENT_TTY].pos_y;

			if TTYS[CURRENT_TTY].pos_x >= BUFFER_WIDTH && byte != b'\n'
			{
				self.new_line();
			}
			match byte
			{
				b'\n' => self.new_line(),
				vga::BACKSPACE  => self.backspace(),
				vga::ESCAPE_START  => self.is_command = true,
				_ =>
				{
					let col = TTYS[CURRENT_TTY].pos_x;

					Writer::buffer().chars[row][col] = vga::ScreenChar {
						character: byte,
						color_code: self.color_code,
					};
					TTYS[CURRENT_TTY].pos_x += 1;
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

	fn clear_row(&mut self, row: usize)
	{
		for col in 0..BUFFER_WIDTH
		{
			Writer::buffer().chars[row][col] = self.blank();
		}
	}

	fn new_line(&mut self)
	{
		unsafe
		{
			TTYS[CURRENT_TTY].pos_x = 0;
			TTYS[CURRENT_TTY].pos_y += 1;
			if TTYS[CURRENT_TTY].pos_y >= BUFFER_HEIGHT
			{
				TTYS[CURRENT_TTY].pos_y = 0;
				TTYS[CURRENT_TTY].has_overflown = true;
			}
			self.clear_row(TTYS[CURRENT_TTY].pos_y);
		}
	}

	fn backspace(&mut self)
	{
		unsafe
		{
			if TTYS[CURRENT_TTY].pos_x > 0
			{
				TTYS[CURRENT_TTY].pos_x -= 1;
				Writer::buffer().chars[TTYS[CURRENT_TTY].pos_y][TTYS[CURRENT_TTY].pos_x] = self.blank();
			}
			else if TTYS[CURRENT_TTY].pos_y > 0 || TTYS[CURRENT_TTY].has_overflown
			{
				TTYS[CURRENT_TTY].pos_x = BUFFER_WIDTH - 1;
				TTYS[CURRENT_TTY].pos_y = if TTYS[CURRENT_TTY].has_overflown
				{
					(TTYS[CURRENT_TTY].pos_y as isize - 1).rem_euclid(BUFFER_HEIGHT as isize) as usize
				}
				else
				{
					TTYS[CURRENT_TTY].pos_y - 1
				}
			}
		}
	}
}

impl Writer // getting static stuff
{
	fn buffer() -> &'static mut Tty
	{
		unsafe { &mut TTYS[CURRENT_TTY] }
	}

	fn blank(&self) -> vga::ScreenChar
	{
		let blank = vga::ScreenChar
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

pub static mut W: Writer = Writer
{
cmd: Escaper {foreground: false, background: false, color: ColorCode::default()},
	is_command: false,
	color_code: ColorCode::default(),
};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
    use core::fmt::Write;

	unsafe { W.write_fmt(args).unwrap(); }
}

fn char_from_input(keyboard_input: &keyboard::KeyboardInput) -> Option<char>
{
	if keyboard_input.state.ctrl
	{
		if keyboard_input.scancode == 0x2E
		{
			shutdown_qemu();
		}
	}
	if keyboard_input.state.shift
	{
		return match keyboard_input.scancode
		{
			0x02 => Some('1'),
			0x03 => Some('2'),
			0x04 => Some('3'),
			0x05 => Some('4'),
			0x06 => Some('5'),
			0x07 => Some('6'),
			0x08 => Some('7'),
			0x09 => Some('8'),
			0x0a => Some('9'),
			0x0b => Some('0'),
			0x0c => Some('°'),
			0x0d => Some('_'),
			0x10 => Some('A'),
			0x0e => Some(0x08 as char),
			0x11 => Some('Z'),
			0x12 => Some('E'),
			0x13 => Some('R'),
			0x14 => Some('T'),
			0x15 => Some('Y'),
			0x16 => Some('U'),
			0x17 => Some('I'),
			0x18 => Some('O'),
			0x19 => Some('P'),
			0x1A => Some('¨'),
			0x1B => Some('*'),
			0x1C => Some('\n'),
			0x1E => Some('Q'),
			0x1F => Some('S'),
			0x20 => Some('D'),
			0x21 => Some('F'),
			0x22 => Some('G'),
			0x23 => Some('H'),
			0x24 => Some('J'),
			0x25 => Some('K'),
			0x26 => Some('L'),
			0x27 => Some('M'),
			0x28 => Some('%'),
			0x29 => Some('>'),
			0x2B => Some('£'),
			0x2C => Some('W'),
			0x2D => Some('X'),
			0x2E => Some('C'),
			0x2F => Some('V'),
			0x30 => Some('B'),
			0x31 => Some('N'),
			0x32 => Some('?'),
			0x33 => Some('.'),
			0x34 => Some('/'),
			0x35 => Some('+'),
			0x39 => Some(' '),
			_ => None,
		};
	}
	else
	{
		return match keyboard_input.scancode
		{
			0x02 => Some('&'),
			0x03 => Some('é'),
			0x04 => Some('"'),
			0x05 => Some('\''),
			0x06 => Some('('),
			0x07 => Some('§'),
			0x08 => Some('è'),
			0x09 => Some('!'),
			0x0a => Some('ç'),
			0x0b => Some('à'),
			0x0c => Some(')'),
			0x0d => Some('-'),
			0x0e => Some(0x08 as char),
			0x10 => Some('a'),
			0x11 => Some('z'),
			0x12 => Some('e'),
			0x13 => Some('r'),
			0x14 => Some('t'),
			0x15 => Some('y'),
			0x16 => Some('u'),
			0x17 => Some('i'),
			0x18 => Some('o'),
			0x19 => Some('p'),
			0x1A => Some('^'),
			0x1B => Some('$'),
			0x1C => Some('\n'),
			0x1E => Some('q'),
			0x1F => Some('s'),
			0x20 => Some('d'),
			0x21 => Some('f'),
			0x22 => Some('g'),
			0x23 => Some('h'),
			0x24 => Some('j'),
			0x25 => Some('k'),
			0x26 => Some('l'),
			0x27 => Some('m'),
			0x28 => Some('ù'),
			0x29 => Some('<'),
			0x2B => Some('`'),
			0x2C => Some('w'),
			0x2D => Some('x'),
			0x2E => Some('c'),
			0x2F => Some('v'),
			0x30 => Some('b'),
			0x31 => Some('n'),
			0x32 => Some(','),
			0x33 => Some(';'),
			0x34 => Some(':'),
			0x35 => Some('='),
			0x39 => Some(' '),
			_ => None,
		};
	}
}

pub fn input(keyboard_input: &keyboard::KeyboardInput)
{
	if let Some(key) = char_from_input(keyboard_input)
	{
		print!("{}{}{}", if keyboard_input.state.ctrl { "^" } else { "" }, key, if keyboard_input.state.ctrl { "\n" } else { "" });
	}
	else
	{
		match keyboard_input.scancode
		{
			0x01 => print!("\x1B"),
			0x4B => handle_arrows(keyboard::Arrow::Left),
			0x4D => handle_arrows(keyboard::Arrow::Right),
			_ => if keyboard_input.scancode & 0x80 == 0
				{
					println!("scancode: {:#x}", keyboard_input.scancode);
				}
		};
	}
	unsafe { TTYS[CURRENT_TTY].print_to_vga(); }
}

fn handle_arrows(arrow: keyboard::Arrow)
{
	unsafe
	{
		match arrow
		{
			keyboard::Arrow::Left => TTYS[CURRENT_TTY].pos_x -= if TTYS[CURRENT_TTY].pos_x > 0 { 1 } else { 0 },
			keyboard::Arrow::Right => TTYS[CURRENT_TTY].pos_x += 1,
		};
		TTYS[CURRENT_TTY].move_cursor();
	}
}
