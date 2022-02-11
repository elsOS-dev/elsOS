use core::arch::asm;
use crate::utilities;
use crate::vga;

pub fn execute(command: &str)
{
	match command
	{
		"help" => help(),
		"clear" => clear(),
		"halt" | "exit" => halt(),
		"reboot" => reboot(),
		"scheen" => scheen(),
		"pm" |"printmem" => printmem(),
		"panic" => panic(),
		"" => {},
		_ => crate::println!("{}: unknown command. Use help for more", command)
	};
}

fn clear()
{
	super::Tty::current().clear();
	vga::Buffer::clear();
}

fn halt()
{
	utilities::shutdown_qemu();
}

fn reboot()
{
	// jump to the post procedure to reboot
	unsafe
	{
		asm!("push 0FFFFh");
		asm!("push 0000h");
		asm!("retf");
	}
}

fn scheen()
{
	crate::println!("yo yo des esch d'becht OS eh ?");
}

fn printmem()
{
	crate::println!("ok");
	crate::serial_println!("==============");
	for i in 0..super::BUFFER_SIZE
	{
		crate::serial_print!("{}", super::Tty::current().chars[i] as char);
	}
	crate::serial_println!("==============");
}

fn panic()
{
	panic!("panic()");
}

fn help()
{
	crate::println!("Supported commands:");
	crate::println!("  help:        show this help");
	crate::println!("  clear:       clear the screen");
	crate::println!("  halt | exit: stop the virtual machine (qemu only)");
	crate::println!("  reboot:      reboot the machine");
	crate::println!("Debug commands:");
	crate::println!("  printmem | pm: print current tty buffer to serial");
	crate::println!("  panic        : trigger a rust panic");
}
