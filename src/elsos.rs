#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![allow(dead_code)]
#![no_std]
#![no_main]

mod tools;
mod vga;
mod keyboard;
mod tty;
mod multiboot;
mod serial;
mod gdt;
mod memory;

use core::panic::PanicInfo;

static VERSION: &str = env!("VERSION");
static PATCHLEVEL: &str = env!("PATCHLEVEL");
static SUBLEVEL: &str = env!("SUBLEVEL");
static EXTRAVERSION: &str = env!("EXTRAVERSION");

use crate::multiboot::MULTIBOOT_MMAP;
use crate::multiboot::MULTIBOOT_MMAP_ENTRIES;

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
		let mut alloc: memory::pageframe::Allocator = memory::pageframe::Allocator::new();
		unsafe { alloc.read_grub_mmap(MULTIBOOT_MMAP, MULTIBOOT_MMAP_ENTRIES); }
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

	logln!("eax: {:08x}   ebx: {:08x}   ecx: {:08x}   edx: {:08x}", eax, ebx, ecx, edx);
	logln!("esi: {:08x}   edi: {:08x}   esp: {:08x}   ebp: {:08x}", esi, edi, esp, ebp);

	log!("\nstack: ");
	for i in 0..24
	{
		unsafe
		{
			log!("{:08x} ", *(esp as *const u32).add(i * 4));
			if (i + 1) % 8 == 0
			{
				log!("\n       ");
			}
		}
	}

	logln!("");
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

