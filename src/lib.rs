#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![feature(llvm_asm)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod utilities;
mod vga_buffer;
mod keyboard;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> !
{
	println!("Hello, kernel world !");
	println!("Welcome to elsos");
	println!("Some numbers: {} and {}", 42, 2.0/3.0);


	keyboard::get_scancodes();
	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	loop {}
}
