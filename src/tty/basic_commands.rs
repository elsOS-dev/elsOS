use core::arch::asm;
use crate::ferramenta;
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
		"pm" => printmem_at(0 as *const u8, false),
		"pb" => printmem_at(0 as *const u8, true),
		"ps" => print_stack(),
		"pt" => printtty(),
		"panic" => panic(),
		"" => {},
		_ =>
		{
			if let Some(command_end) = command.find(' ')
			{
				let arg = &command[command_end + 1..];
				let command = &command[..command_end];
				match command
				{
					"pm" | "pb" =>
					{
						let address = u32::from_str_radix(arg, 16)
													 .unwrap_or_else(|_|
														{
															crate::println!("invalid argument for {}: {}",
																			command, arg);
															0
														});
						match command
						{
							"pm" => printmem_at(address as *const u8, false),
							"pb" => printmem_at(address as *const u8, true),
							_ => {}
						};
					}
					_ => crate::println!("{}: unknown or invalid command. Use help for more", command)
				}
			}
			else
			{
				crate::println!("{}: unknown or invalid command. Use help for more", command);
			}
		}
	};
}

fn clear()
{
	super::Tty::current().clear();
	vga::Buffer::clear();
}

fn halt()
{
	ferramenta::shutdown_qemu();
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

fn printtty()
{
	crate::println!("ok");
	crate::serial_println!("==============");
	for i in 0..super::BUFFER_SIZE
	{
		crate::serial_print!("{}", super::Tty::current().chars[i] as char);
	}
	crate::serial_println!("==============");
}

fn printmem_at(address: *const u8, binary: bool)
{
	unsafe
	{
		if binary
		{
			ferramenta::print_memory_bin(address, 256);
		}
		else
		{
			ferramenta::print_memory(address, 256);
		}
	}
}

fn print_stack()
{
	unsafe
	{
		ferramenta::print_memory(crate::get_reg!("esp") as *const u8, 10 * 16);
	}
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
	crate::println!("  pm <address>: print 256 bytes of memory at address (0 if not specified)");
	crate::println!("  pb <address>: |-------------- same in binary");
	crate::println!("  ps:           print stack");
	crate::println!("  pt:           print current tty buffer to serial");
	crate::println!("  panic:        trigger a rust panic");
}
