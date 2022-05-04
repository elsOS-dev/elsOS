#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![allow(dead_code)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::multiboot::{MULTIBOOT_MMAP, MULTIBOOT_MMAP_ENTRIES};

mod ferramenta;
mod vga;
mod keyboard;
mod tty;
mod multiboot;
mod serial;
mod gdt;
mod memory;
mod libc;

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
		unsafe { crate::memory::init(MULTIBOOT_MMAP, MULTIBOOT_MMAP_ENTRIES); }
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
	logln!("\n\x1B[31;49m{}\x1B[39;49m\n", info);

	print_memory_state(false);

	logln!("");
	loop {}
}

fn print_memory_state(serial_only: bool)
{
	let eax: u32;
	let ebx: u32;
	let ecx: u32;
	let edx: u32;

	let esi: u32;
	let edi: u32;
	let esp: u32;
	let ebp: u32;

	unsafe
	{
		eax = crate::get_reg!("eax");
		ebx = crate::get_reg!("ebx");
		ecx = crate::get_reg!("ecx");
		edx = crate::get_reg!("edx");

		esi = crate::get_reg!("esi");
		edi = crate::get_reg!("edi");
		esp = crate::get_reg!("esp");
		ebp = crate::get_reg!("ebp");
	}

	if !serial_only
	{
		crate::vga_println!("eax: {:08x}   ebx: {:08x}   ecx: {:08x}   edx: {:08x}", eax, ebx, ecx, edx);
		crate::vga_println!("esi: {:08x}   edi: {:08x}   esp: {:08x}   ebp: {:08x}", esi, edi, esp, ebp);
	}
	crate::serial_println!("eax: {:08x}   ebx: {:08x}   ecx: {:08x}   edx: {:08x}", eax, ebx, ecx, edx);
	crate::serial_println!("esi: {:08x}   edi: {:08x}   esp: {:08x}   ebp: {:08x}", esi, edi, esp, ebp);

	if !serial_only
	{
		crate::vga_print!("\nstack: ");
	}
	crate::serial_print!("\nstack: ");
	for i in 0..24
	{
		unsafe
		{
			if !serial_only
			{
				crate::vga_print!("{:08x} ", *(esp as *const u32).add(i * 4));
			}
			crate::serial_print!("{:08x} ", *(esp as *const u32).add(i * 4));
			if (i + 1) % 8 == 0
			{
				if !serial_only
				{
					crate::vga_print!("\n       ");
				}
				crate::serial_print!("\n       ");
			}
		}
	}
}

#[macro_export]
macro_rules! oops
{
	($($arg:tt)*) =>
	{
		$crate::logln!("\x1B[33;49moops at '{}', {}:{}:{}\x1B[39;49m\n", format_args!($($arg)*), file!(), line!(), column!());
		$crate::print_memory_state(true);
		$crate::serial_println!("");
	}
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

