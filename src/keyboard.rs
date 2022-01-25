use core::arch::asm;
use crate::println;
use crate::print;

#[inline(always)]
pub fn inb(port: u32) -> u8
{
    let ret: u8;
    unsafe
    {
        asm!("in al, dx", out("al") ret, in("dx") port,
                options(nomem, nostack, preserves_flags));
    }
    ret
}


pub fn outb(port: u32, value: u8)
{
    unsafe
    {
        asm!("out dx, al", in("dx") port, in("al") value,
                options(nomem, nostack, preserves_flags));
    }
}

const KEYBOARD_DATA: u32 = 0x60;
const KEYBOARD_READ_STATUS: u32 = 0x64;
const KEYBOARD_WRITE_COMMAND: u32 = 0x64;

struct KeyboardStates
{
    is_shift: bool,
    is_ctrl: bool,
}

static mut keyboard_states: KeyboardStates = KeyboardStates
{
    is_shift: false,
    is_ctrl: false,
};

pub fn get_scancodes()
{
	let mut scancode: u8 = 0;
	loop
    {
		let new_scancode = inb(KEYBOARD_DATA);

		if new_scancode == scancode
		{
			continue;
		}
		scancode = new_scancode;
		let key = match scancode
		{
			0x02 => unsafe { if keyboard_states.is_shift { Some('1') } else { Some('&') } },
			0x03 => Some('2'),
			0x04 => Some('3'),
			0x05 => Some('4'),
			0x06 => Some('5'),
			0x07 => Some('6'),
			0x08 => Some('7'),
			0x09 => Some('8'),
			0x0a => Some('9'),
			0x0b => Some('0'),
			0x0c => Some(')'),
			0x0d => Some('-'),
			// 0x0e => Some(''), Backspace
			// 0x0f => Some(''),
			0x10 => Some('a'),
			0x11 => Some('z'),
			0x12 => Some('e'),
			0x13 => Some('r'),
			0x14 => Some('t'),
			0x15 => Some('y'),
			0x16 => Some('u'),
			0x17 => Some('i'),
			0x18 => Some('o'),
			0x19 => unsafe { if keyboard_states.is_shift { Some('P') } else { Some('p') } },
			0x1A => Some('^'),
			0x1B => Some('$'),
			0x1C => Some('\n'),
			//0x1D => Some(''), ctrl gauche
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
			0x29 => Some('`'),
			//0x2A => Some(''),
			//0x2B => Some(''),
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
			// 0x36 => Some(''), Shift droite
			// 0x37 => Some(''),
			// 0x38 => Some(''), alt gauche
			0x39 => Some(' '),
			_ => None,
	    };

		if let Some(key) = key
		{
            unsafe
            {
			    print!("{}{}{}", if keyboard_states.is_ctrl { "^" } else { "" }, key, if keyboard_states.is_ctrl { "\n" } else { "" });
            }
		}
		else
		{
            if scancode % 0x80 == 0x2a
            {
                unsafe
                {
                    keyboard_states.is_shift = !keyboard_states.is_shift;
                }
            }
            if scancode % 0x80 == 0x1d
            {
                unsafe
                {
                    keyboard_states.is_ctrl = !keyboard_states.is_ctrl;
                }
            }
			if scancode & 0x80 == 0
			{
				println!("scancode: {:#x}", scancode);
			}
		}
	}
}
