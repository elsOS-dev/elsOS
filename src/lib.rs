#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![feature(llvm_asm)]
#![no_std]
#![no_main]

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
		//llvm_asm!("inb %%dx,%%al":"=a" (ret):"d" (port));
		asm!("inb {}, {}", out(reg_byte) ret, in(reg) port);
	}
	ret
}


pub fn outb(port: u32, value: u8)
{
	unsafe
	{
		llvm_asm!("outb %%al,%%dx": :"d" (port), "a" (value));
	}
}



#[no_mangle]
pub extern "C" fn kernel_main() -> !
{
	println!("Hello, kernel world !");
	println!("Welcome to elsos");
	println!("Some numbers: {} and {}", 42, 2.0/3.0);
	loop
   	{
		let scancode = inb(0x60);
		println!("scancode: {}", scancode);
	}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	loop {}
}
