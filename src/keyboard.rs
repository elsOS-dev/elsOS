use crate::println;
use crate::print;
use crate::utilities::shutdown_qemu;

const KEYBOARD_DATA: u32 = 0x60;
const KEYBOARD_READ_STATUS: u32 = 0x64;
const KEYBOARD_WRITE_COMMAND: u32 = 0x64;

struct KeyboardStates
{
    is_shift: bool,
    is_ctrl: bool,
}

static mut KEYBOARD_STATES: KeyboardStates = KeyboardStates
{
    is_shift: false,
    is_ctrl: false,
};

fn char_from_scancode(scancode: u8) -> Option<char>
{
	unsafe
	{
		if KEYBOARD_STATES.is_ctrl
		{
			if scancode == 0x2E
			{
				shutdown_qemu();
			}
		}
		if KEYBOARD_STATES.is_shift
		{
			return match scancode
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
			return match scancode
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
}

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

		if let Some(key) = char_from_scancode(scancode)
		{
            unsafe
            {
			    print!("{}{}{}", if KEYBOARD_STATES.is_ctrl { "^" } else { "" }, key, if KEYBOARD_STATES.is_ctrl { "\n" } else { "" });
            }
		}
		else
		{
			unsafe
			{
				match scancode
				{
					0x2A => KEYBOARD_STATES.is_shift = true,
					0xAA => KEYBOARD_STATES.is_shift = false,
					0x1D => KEYBOARD_STATES.is_ctrl = true,
					0x9D => KEYBOARD_STATES.is_ctrl = false,
					_ => if scancode % 0x80 == 0
					{
						println!("scancode: {:#x}", scancode);
					}
				};
			}
		}
	}
}
