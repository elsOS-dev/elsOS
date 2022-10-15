use crate::arch;
use crate::tty;

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

pub fn char_from_input(keyboard_input: &KeyboardInput) -> Option<char>
{
	unsafe
	{
		if crate::SETTINGS.layout == 1
		{
			qwerty::char_from_input(keyboard_input)
		}
		else
		{
			azerty::char_from_input(keyboard_input)
		}
	}
}

pub fn get_scancodes()
{
	let mut scancode: u8 = 0;
	loop
    {
		let new_scancode = arch::port::inb(KEYBOARD_DATA);

		if new_scancode == scancode
		{
			continue;
		}
		scancode = new_scancode;

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
}
