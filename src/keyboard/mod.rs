use crate::arch;
use crate::tty;

use core::mem::MaybeUninit;
use alloc::string::String;

mod azerty;
mod qwerty;

const KEYBOARD_DATA: u16 = 0x60;
const KEYBOARD_READ_STATUS: u32 = 0x64;
const KEYBOARD_WRITE_COMMAND: u32 = 0x64;

pub enum Arrow
{
	Left,
	Right,
	Up,
	Down
}

pub struct KeyboardInput
{
	pub state: KeyboardState,
	pub scancode: u8,
}

pub struct KeyboardState
{
    pub shift: bool,
    pub ctrl: bool,
}

static mut KEYBOARD_STATE: KeyboardState = KeyboardState
{
    shift: false,
    ctrl: false,
};


pub static mut BUFFER: MaybeUninit<String> = MaybeUninit::uninit();

pub fn char_from_input(keyboard_input: &KeyboardInput) -> Option<char>
{
	unsafe
	{
		let c = if crate::SETTINGS.layout == 1
		{
			qwerty::char_from_input(keyboard_input)
		}
		else
		{
			azerty::char_from_input(keyboard_input)
		};
		if let Some(key) = c
		{
			let buf = BUFFER.assume_init_mut();
			buf.push(key);
		}
		c
	}
}

pub fn get_scancode()
{
	let scancode = arch::port::inb(KEYBOARD_DATA);

	unsafe
	{
		match scancode
		{
			0x2A => KEYBOARD_STATE.shift = true,
			0xAA => KEYBOARD_STATE.shift = false,
			0x1D => KEYBOARD_STATE.ctrl = true,
			0x9D => KEYBOARD_STATE.ctrl = false,
			_ => {
				tty::input(&KeyboardInput {
					state: KeyboardState {
						shift: KEYBOARD_STATE.shift,
						ctrl: KEYBOARD_STATE.ctrl
					},
					scancode,
				});
			}
		};
	}
}
