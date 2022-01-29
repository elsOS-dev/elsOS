use crate::terminal;

const KEYBOARD_DATA: u32 = 0x60;
const KEYBOARD_READ_STATUS: u32 = 0x64;
const KEYBOARD_WRITE_COMMAND: u32 = 0x64;

pub enum Arrow
{
	Left,
	Right
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

pub fn get_scancodes()
{
	let mut scancode: u8 = 0;
	loop
    {
		let new_scancode = crate::utilities::inb(KEYBOARD_DATA);

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
					terminal::input(&KeyboardInput {
						state: KeyboardState {
							shift: KEYBOARD_STATE.shift,
							ctrl: KEYBOARD_STATE.ctrl
						},
						scancode: scancode,
					});
				}
			};
		}
	}
}
