#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod utilities;
mod vga;
mod keyboard;
mod tty;
mod multiboot;
mod serial;
mod gdt;

use core::panic::PanicInfo;

static VERSION: &str = env!("VERSION");
static PATCHLEVEL: &str = env!("PATCHLEVEL");
static SUBLEVEL: &str = env!("SUBLEVEL");
static EXTRAVERSION: &str = env!("EXTRAVERSION");

pub struct Settings
{
	has_serial: bool,
	layout: u8
}

pub static mut SETTINGS: Settings = Settings
{
	has_serial: false,
	layout: 0
};

#[no_mangle]
pub extern "C" fn kernel_main(magic: u32, address: u32)
{

	gdt::init_gdt();
	init_vga();
	vga::cursor::Cursor::init(0, 15);
	if multiboot::check_magic(magic) && multiboot::parse(address)
	{
		init_serial();
		logln!("\n");
		logln!("        :::      ::::::::    __       __       __ _  ____  ____  ");
		logln!("      :+:      :+:    :+:  .'  `'._.'`  '.    (  / )(  __)/ ___) ");
		logln!("    +:+ +:+         +:+   |  .--;   ;--.  |   |   ( |  _) \\___ \\ ");
		logln!("  +#+  +:+       +#+      |  (  /   \\  )  |   (__\\_)(__)  (____/ ");
		logln!("+#+#+#+#+#+   +#+          \\  ;` /^\\ `;  /    ");
		logln!("     #+#    #+#             :` .'._.'. `;    Willkumme uf elsOS {}.{}.{}{}", VERSION, PATCHLEVEL, SUBLEVEL, EXTRAVERSION);
		logln!("    ###   #########         '-`'.___.'`-'   Hello, kernel world !");
		logln!();
		tty::prompt();
		keyboard::get_scancodes();
	}
}

fn init_vga()
{
	vga::Buffer::clear();
}

fn init_serial()
{
	crate::println!("[{}] init serial", ok_fail(serial::init(serial::COM1)));
}

pub fn ok_fail(value: bool) -> &'static str
{
	match value
	{
		true => "\x1B[32m OK \x1B[39m",
		false => "\x1B[31mFAIL\x1B[39m"
	}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	vga::panic();
	logln!("\n\x1B[31;49m{}\x1B[39;49m", info);
	loop {}
}

#[macro_export]
macro_rules! log
{
	($($arg:tt)*) =>
	{
		($crate::serial::_print(format_args!($($arg)*)));
		($crate::tty::_print(format_args!($($arg)*)));
	}
}

#[macro_export]
macro_rules! logln
{
	() => ($crate::log!("\n"));
	($($arg:tt)*) => ($crate::log!("{}\n", format_args!($($arg)*)));
}
