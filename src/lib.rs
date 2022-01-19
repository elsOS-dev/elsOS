#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![feature(llvm_asm)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod utilities;
mod vga_buffer;
//extern crate rlibc;
use core::panic::PanicInfo;
use core::arch::asm;

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

#[no_mangle]
pub extern "C" fn kernel_main() -> !
{
	println!("Hello, kernel world !");
	println!("Welcome to elsos");
	println!("Some numbers: {} and {}", 42, 2.0/3.0);
	loop
   	{
		let scancode = inb(KEYBOARD_DATA);
		let status =  inb(KEYBOARD_READ_STATUS);

		let _buffer_status = utilities::get_bit_at(status, 0);

		if scancode & 0x80 == 0
		{
			println!("scancode: {}", scancode);
		}
	}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	loop {}
}
