#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod utilities;
mod vga;
mod keyboard;
mod terminal;

use core::panic::PanicInfo;
use crate::vga::cursor::init_cursor;

static VERSION: &str = "0";
static PATCHLEVEL: &str = "0";
static SUBLEVEL: &str = "1";
static EXTRAVERSION: &str = "";

static MULTIBOOT2_BOOTLOADER_MAGIC: u32 = 0x36d76289;

#[no_mangle]
pub extern "C" fn kernel_main(magic: u32, address: u32) -> !
{
	if check_multiboot2_magic(magic)
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

fn check_multiboot2_magic(magic: u32) -> bool
{
	let magic_ok = magic == MULTIBOOT2_BOOTLOADER_MAGIC;

	print!("[{}] multiboot2 magic number", ok_fail(magic_ok));
	if !magic_ok
	{
		print!(": expected {:#0x}, got {:#0x}.", MULTIBOOT2_BOOTLOADER_MAGIC, magic);
	}
	print!("\n");

	magic_ok
}

fn ok_fail(value: bool) -> &'static str
{
	if value
	{
		return "\x1B32m OK \x1B38m";
	}
	else
	{
		return "\x1B31mFAIL\x1B38m";
	}
}
