#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

mod vga_buffer;
extern crate rlibc;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> !
{
	println!("Hello, kernel world !");
	println!("Welcome to elsos");
	println!("Some numbers: {} and {}", 42, 2.0/3.0);
	loop
	{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
	loop
	{}
}
