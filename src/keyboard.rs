use core::arch::asm;
use crate::println;

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
		if scancode & 0x80 == 0
		{
			println!("scancode: {}", scancode);
		}
	}

}
