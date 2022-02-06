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
mod serial;

use core::panic::PanicInfo;

static VERSION: &str = env!("VERSION");
static PATCHLEVEL: &str = env!("PATCHLEVEL");
static SUBLEVEL: &str = env!("SUBLEVEL");
static EXTRAVERSION: &str = env!("EXTRAVERSION");

#[no_mangle]
pub extern "C" fn kernel_main(magic: u32, address: u32)
{
	init_serial();
	vga::cursor::init(0, 15);
	if multiboot::check_magic(magic) && multiboot::parse(address)
	{
		logln!("\n");
		logln!("        :::      ::::::::    __       __       __ _  ____  ____  ");
		logln!("      :+:      :+:    :+:  .'  `'._.'`  '.    (  / )(  __)/ ___) ");
		logln!("    +:+ +:+         +:+   |  .--;   ;--.  |   |   ( |  _) \\___ \\ ");
		logln!("  +#+  +:+       +#+      |  (  /   \\  )  |   (__\\_)(__)  (____/ ");
		logln!("+#+#+#+#+#+   +#+          \\  ;` /^\\ `;  /    ");
		logln!("     #+#    #+#             :` .'._.'. `;    Willkumme uf elsOS {}.{}.{}{}", VERSION, PATCHLEVEL, SUBLEVEL, EXTRAVERSION);
		logln!("    ###   #########         '-`'.___.'`-'   Hello, kernel world !");

		print!("\x1B41;32mHenlo\x1B38;48m");
		keyboard::get_scancodes();
	}
}

fn init_serial()
{
	println!("[{}] init serial", boot::ok_fail(serial::init(serial::COM1)));
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	logln!("{}", info);
	loop {}
}

#[macro_export]
macro_rules! log
{
	($($arg:tt)*) => ($crate::tty::_print(format_args!($($arg)*)));
	($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! logln
{
	() =>
	{
		($crate::print!("\n"));
		($crate::serial_print!("\n"));
	};
	($($arg:tt)*) =>
	{
		($crate::print!("{}\n", format_args!($($arg)*)));
		($crate::serial_print!("{}\n", format_args!($($arg)*)));
	}
}
