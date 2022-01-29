#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![feature(llvm_asm)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod utilities;
mod vga;
mod keyboard;
mod terminal;

use core::panic::PanicInfo;
use crate::vga::init_cursor;

static VERSION: &str = "0";
static PATCHLEVEL: &str = "0";
static SUBLEVEL: &str = "1";
static EXTRAVERSION: &str = "";

#[no_mangle]
pub extern "C" fn kernel_main() -> !
{
	init_cursor(0, 15);
	println!("        :::      ::::::::");
	println!("      :+:      :+:    :+:");
	println!("    +:+ +:+         +:+  ");
	println!("  +#+  +:+       +#+     ");
	println!("+#+#+#+#+#+   +#+        ");
	println!("     #+#    #+#          ");
	println!("    ###   #########      \n\n");
	println!("Hello, kernel world !");
	println!("Willkumme uf elsOS {}.{}.{}{}\n", VERSION, PATCHLEVEL, SUBLEVEL, EXTRAVERSION);

	keyboard::get_scancodes();
	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	println!("{}", info);
	loop {}
}
