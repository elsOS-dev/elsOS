use crate::keyboard;
use crate::vga;

mod basic_commands;
mod print;

pub use crate::tty::print::_print;

const BUFFER_HEIGHT: usize = vga::BUFFER_HEIGHT;
const BUFFER_WIDTH: usize = vga::BUFFER_WIDTH;
const BUFFER_SIZE: usize = BUFFER_WIDTH * BUFFER_HEIGHT;
const INPUT_SIZE: usize = BUFFER_SIZE / 2;

static mut VGA: *mut vga::Buffer = 0xb8000 as *mut vga::Buffer;

#[derive(Copy, Clone)]
#[repr(C)]
struct Tty
{
	has_init: bool,
	chars: [u8; BUFFER_SIZE],
	pos: usize,
	pos_offset: usize,
	cursor_offset: usize,
	scroll: usize,
	input: [u8; INPUT_SIZE],
	command: &'static str
}

impl Tty
{
	fn prompt(&mut self)
	{
		if !self.has_init
		{
			self.has_init = true;
			unsafe
			{
				crate::println!("elsOS tty{}\n", CURRENT_TTY + 1);
			}
		}
		crate::print!("\x1B[30;47mWas esch los ? >\x1B[39;49m ");
	}

	fn execute(&mut self)
	{
		basic_commands::execute(self.command);
		self.prompt();
	}

	fn increase_pos_with_offset(&mut self)
	{
		let tmp_offset = self.pos_offset;
		self.pos_offset = 0;
		self.increase_pos_by(tmp_offset);
	}

	fn remove_first_line(&mut self)
	{
		let mut len_to_remove: usize = 0;

		for byte in &Tty::current().chars[..]
		{
			len_to_remove += 1;
			if *byte == b'\n'
			{
				break;
			}
		}
		for i in 0..len_to_remove
		{
			Tty::current().chars[i] = b'\0';
		}
		self.chars.rotate_left(len_to_remove);
		self.pos -= len_to_remove;
	}

	fn increase_pos_by(&mut self, offset: usize)
	{
		if self.pos + self.pos_offset + offset >= BUFFER_SIZE
		{
			self.remove_first_line();
		}
		self.pos += offset;
	}

	fn current() -> &'static mut Tty
	{
		unsafe { &mut TTYS[CURRENT_TTY] }
	}

	fn clear(&mut self)
	{
		self.chars.fill(b'\0');
		self.pos = 0;
		self.pos_offset = 0;
		self.cursor_offset = 0;
		self.scroll = 0;
		self.input.fill(b'\0');
		self.command = "";
	}
}

pub fn prompt()
{
	unsafe
	{
		TTYS[CURRENT_TTY].prompt();
	}
}

static mut CURRENT_TTY: usize = 0;
static mut TTYS: [Tty; 8] =
[
	Tty
	{
		has_init: false,
		chars: [b'\0'; BUFFER_SIZE],
		pos: 0,
		pos_offset: 0,
		cursor_offset: 0,
		scroll: 0,
		input: [b'\0'; INPUT_SIZE],
		command: ""
	}; 8
];

pub fn write_byte(byte: u8, offset: bool)
{
	unsafe
	{
		if Tty::current().pos + Tty::current().pos_offset >= BUFFER_SIZE
		{
			Tty::current().remove_first_line();
		}
		let pos = Tty::current().pos + Tty::current().pos_offset - Tty::current().cursor_offset;
		if Tty::current().cursor_offset > 0
		{
			let right_part: &mut [u8] = &mut Tty::current().chars[pos..];
			right_part.rotate_right(1);
		}
		Tty::current().chars[pos] = byte;
		if offset
		{
			TTYS[CURRENT_TTY].pos_offset += 1;
		}
		else
		{
			TTYS[CURRENT_TTY].pos += 1;
		}
	}
}

pub fn input(keyboard_input: &keyboard::KeyboardInput)
{
	if let Some(key) = keyboard::char_from_input(keyboard_input)
	{
		if keyboard_input.state.ctrl
		{
			match key
			{
				'C' | 'c' => {
					write_byte(b'^', true);
					write_byte(b'C', true);
					print::print_to_vga();
					line_return(false);
					prompt();
					return;
				},
				_ => {}
			};
		}
		if Tty::current().pos_offset < BUFFER_SIZE / 2
		{
			write_byte(key as u8, true);
			if Tty::current().scroll > 0
			{
				unsafe
				{
					vga::W.scroll = 0;
					print::print_to_vga();
				}
			}
			else if Tty::current().cursor_offset > 0
			{
				print::print_to_vga();
			}
			else
			{
				print::print_byte_to_vga(key as u8);
			}
		}
	}
	else
	{
		match keyboard_input.scancode
		{
			0x0e => backspace(),
			0x1C => line_return(true),
			0x4B => cursor_left(),
			0x4D => cursor_right(),
			0x48 => cursor_up(),
			0x50 => cursor_down(),
			0x3B..=0x42 => handle_tty_change((keyboard_input.scancode - 0x3B).into()),
			_ => if keyboard_input.scancode & 0x80 == 0
				{
					crate::serial_println!("scancode: {:#x}", keyboard_input.scancode);
				}
		};
	}
}

fn cursor_left()
{
	if Tty::current().pos_offset - Tty::current().cursor_offset > 0
	{
		let position = vga::cursor::Cursor::get_position();

		Tty::current().cursor_offset += 1;
		unsafe
		{
			vga::cursor::CURSOR.offset = Tty::current().cursor_offset;
		}
		vga::cursor::Cursor::move_to(position.0, position.1);
	}
}

fn cursor_right()
{
	if Tty::current().cursor_offset > 0
	{
		Tty::current().cursor_offset -= 1;
		unsafe
		{
			vga::cursor::CURSOR.offset -= 1;
		}

		let position = vga::cursor::Cursor::get_position();
		vga::cursor::Cursor::move_to(position.0 + 1, position.1);
	}
}

fn cursor_down()
{
	if vga::scroll_down()
	{
		Tty::current().scroll -= 1;
		print::print_to_vga();
	}
}

fn cursor_up()
{
	if vga::scroll_up()
	{
		Tty::current().scroll += 1;
		print::print_to_vga();
	}
}

fn handle_tty_change(tty: usize)
{
	unsafe
	{
		CURRENT_TTY = tty;
		if !Tty::current().has_init
		{
			Tty::current().prompt();
		}
		vga::cursor::CURSOR.offset = Tty::current().cursor_offset;
		vga::W.scroll = Tty::current().scroll;
		print::print_to_vga();
	}
}

fn line_return(execute: bool)
{
	let input_start = Tty::current().pos;
	let input_len = Tty::current().pos_offset;
	Tty::current().input.fill(b'\0');
	Tty::current().input[..input_len].copy_from_slice(&Tty::current().chars[input_start..input_start + input_len]);
	Tty::current().command = &core::str::from_utf8(&Tty::current().input).unwrap()[..input_len];
	Tty::current().increase_pos_with_offset();
	Tty::current().increase_pos_by(1);
	Tty::current().cursor_offset = 0;
	unsafe
	{
		vga::cursor::CURSOR.offset = 0;
	}
	crate::println!();
	if execute
	{
		Tty::current().execute();
	}
}

fn backspace()
{
	if Tty::current().pos_offset - Tty::current().cursor_offset > 0
	{
		Tty::current().pos_offset -= 1;

		let pos = Tty::current().pos + Tty::current().pos_offset - Tty::current().cursor_offset;

		Tty::current().chars[pos] = b'\0';
		if Tty::current().cursor_offset > 0
		{
			let right_part: &mut [u8] = &mut Tty::current().chars[pos..];
			right_part.rotate_left(1);
		}
		print::print_to_vga();
	}
}
