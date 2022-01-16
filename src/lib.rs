#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

mod vga_buffer;
extern crate rlibc;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> !
{
	vga_buffer::write("Hello, elsos world !");
	loop
	{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
	loop
	{}
}
