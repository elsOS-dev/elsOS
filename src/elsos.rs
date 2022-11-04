#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![allow(dead_code)]
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use crate::multiboot::{MULTIBOOT_MMAP, MULTIBOOT_MMAP_ENTRIES};

use alloc::string::String;

mod arch;
mod ferramenta;
mod vga;
mod keyboard;
mod tty;
mod multiboot;
mod serial;
mod memory;
mod libc;
mod time;
mod syscall;

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
	arch::initial::init();
	init_vga();
	vga::cursor::Cursor::init(0, 15);
	if multiboot::check_magic(magic) && multiboot::parse(address)
	{
		init_serial();
		init_memory();
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
		unsafe
		{
			keyboard::BUFFER.as_mut_ptr().write(String::new());
			arch::interrupts::init();
			arch::interrupts::enable();
		}
		tests();
		loop
		{
			arch::halt();
		}
	}
}

fn tests()
{
	// put tests here
	vga_println!("getting one line...");
	vga_println!("got line \"{}\"", ferramenta::get_line());
	unsafe
	{
		let mut text = [b'H', b'e', b'l', b'l', b'o'];
		let len = 5;
		syscall::write(0, text.as_ptr() as u32, len);
		vga_println!();
		syscall::read(0, text.as_mut_ptr() as u32, len);
		syscall::write(0, text.as_ptr() as u32, len);
		vga_println!();
	}
}

fn init_vga()
{
	vga::Buffer::clear();
}

fn init_serial()
{
	crate::println!("[{}] initialized serial", ok_fail(serial::init(serial::COM1)));
}

fn init_memory()
{
	unsafe
	{
		crate::memory::init(MULTIBOOT_MMAP, MULTIBOOT_MMAP_ENTRIES);
		crate::println!("[{}] initilized memory", ok_fail(true));
	}
}

pub fn ok_fail(value: bool) -> &'static str
{
	match value
	{
		true => "\x1B[32m OK \x1B[39m",
		false => "\x1B[31mFAIL\x1B[39m"
	}
}

static mut INTERRUPT_STATE: arch::interrupts::State = arch::interrupts::State
{
	eax: 0,
	ebx: 0,
	ecx: 0,
	edx: 0,

	esi: 0,
	edi: 0,
	esp: 0,
	ebp: 0,

	cs: 0,
	ds: 0,

	interrupt: 0,
	error: 0,

	eip: 0,
	eflags: 0,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
	vga::panic();
	logln!("\n\x1B[31;49m{}\x1B[39;49m\n", info);

	print_panic_state(false);

	logln!("");
	unsafe
	{
		clear_reg!("eax");
		clear_reg!("ebx");
		clear_reg!("ecx");
		clear_reg!("edx");

		clear_reg!("esi");
		clear_reg!("edi");
		clear_reg!("esp");
		clear_reg!("ebp");
	}
	loop {}
}

fn print_panic_state(serial_only: bool)
{
	let eip: u32;
	let eflags: u32;

	let eax: u32;
	let ebx: u32;
	let ecx: u32;
	let edx: u32;

	let esi: u32;
	let edi: u32;
	let esp: u32;
	let ebp: u32;

	let cs: u32;
	let ds: u32;

	unsafe
	{
		eip = INTERRUPT_STATE.eip;
		eflags = INTERRUPT_STATE.eflags;
		eax = INTERRUPT_STATE.eax;
		ebx = INTERRUPT_STATE.ebx;
		ecx = INTERRUPT_STATE.ecx;
		edx = INTERRUPT_STATE.edx;

		esi = INTERRUPT_STATE.esi;
		edi = INTERRUPT_STATE.edi;
		esp = INTERRUPT_STATE.esp;
		ebp = INTERRUPT_STATE.ebp;

		cs = INTERRUPT_STATE.cs;
		ds = INTERRUPT_STATE.ds;
	}

	if !serial_only
	{
		crate::vga_println!("eip: {:08x}   eflags: {:08x}", eip, eflags);
		crate::vga_println!("eax: {:08x}   ebx: {:08x}   ecx: {:08x}   edx: {:08x}", eax, ebx, ecx, edx);
		crate::vga_println!("esi: {:08x}   edi: {:08x}   esp: {:08x}   ebp: {:08x}", esi, edi, esp, ebp);
		crate::vga_println!(" cs:     {:04x}    ds:     {:04x}", cs, ds);
	}
	crate::serial_println!("eip: {:08x}   eflags: {:08x}", eip, eflags);
	crate::serial_println!("eax: {:08x}   ebx: {:08x}   ecx: {:08x}   edx: {:08x}", eax, ebx, ecx, edx);
	crate::serial_println!("esi: {:08x}   edi: {:08x}   esp: {:08x}   ebp: {:08x}", esi, edi, esp, ebp);
	crate::serial_println!(" cs:     {:04x}    ds:     {:04x}", cs, ds);

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
	log!("\n\nALLES KAPUT !!!");
}

#[macro_export]
macro_rules! oops
{
	($($arg:tt)*) =>
	{
		$crate::logln!("\x1B[33;49moops at '{}', {}:{}:{}\x1B[39;49m\n", format_args!($($arg)*), file!(), line!(), column!());
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
