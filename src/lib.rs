#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod boot;
mod utilities;
mod vga;
mod keyboard;
mod tty;
mod multiboot;

use core::panic::PanicInfo;

static VERSION: &str = "0";
static PATCHLEVEL: &str = "0";
static SUBLEVEL: &str = "1";
static EXTRAVERSION: &str = "";

#[no_mangle]
pub extern "C" fn kernel_main(magic: u32, address: u32) -> !
{
	vga::cursor::init(0, 15);
	if multiboot::check_magic(magic) && multiboot::parse(address)
	{
		println!("\n");
		println!("        :::      ::::::::");
		println!("      :+:      :+:    :+:");
		println!("    +:+ +:+         +:+       Hello, kernel world !");
		println!("  +#+  +:+       +#+     ");
		println!("+#+#+#+#+#+   +#+             Willkumme uf elsOS {}.{}.{}{}", VERSION, PATCHLEVEL, SUBLEVEL, EXTRAVERSION);
		println!("     #+#    #+#          ");
		println!("    ###   #########      ");

		print!("\x1B41;32mHenlo\x1B38;48m");
		keyboard::get_scancodes();
	}

	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	loop {}
}
