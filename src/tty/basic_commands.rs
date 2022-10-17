use core::ffi::c_void;
use crate::arch;
use crate::ferramenta;
use crate::memory;
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
		"jiffies" => jiffies(),
		"yesss" => yesss(),
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
					"pm" | "pb" | "pk" | "pv" | "km" | "vm" | "kfree" | "vfree" =>
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
							"pk" => print_var(address as *const u8, true),
							"pv" => print_var(address as *const u8, false),
							"km" => allocate(address as usize, true),
							"vm" => allocate(address as usize, false),
							"kfree" => free(address as *mut c_void, true),
							"vfree" => free(address as *mut c_void, false),
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

// use the 8042 keyboard controller to pulse the reset pin of the CPU
fn reboot()
{
	let mut good: u8 = 0x02;
	while good & 0x02 != 0
	{
		good = arch::port::inb(0x64);
	}
	arch::port::outb(0x64, 0xFE);
	arch::halt();
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
			ferramenta::print_memory(address, 0x1000);
		}
	}
}

fn print_var(address: *const u8, kernel_space: bool)
{
	unsafe
	{
		let size = if kernel_space
		{
			memory::ksize(address as *mut c_void)
		}
		else
		{
			memory::vsize(address as *mut c_void)
		};
		if size > 0
		{
			let address = (address as usize - 0x10) as *const u8;
			ferramenta::print_memory(address, size);
		}
	}
}

fn allocate(size: usize, kernel_space: bool)
{
	let address = if kernel_space
	{
		memory::kmalloc(size)
	}
	else
	{
		memory::vmalloc(size)
	};
	if address.is_null()
	{
		return;
	}
	let size = if kernel_space
	{
		memory::ksize(address)
	}
	else
	{
		memory::vsize(address)
	};
	crate::logln!("allocated {} ({:#0x}) bytes at {:#08x}", size, size, address as usize);
}

fn free(address: *mut c_void, kernel_space: bool)
{
	let size = if kernel_space
	{
		memory::ksize(address)
	}
	else
	{
		memory::vsize(address)
	};
	if size > 0
	{
		if kernel_space
		{
			memory::kfree(address);
		}
		else
		{
			memory::vfree(address);
		}
		crate::logln!("freeed {} ({:#0x}) bytes at {:#08x}", size, size, address as usize);
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

fn jiffies()
{
	unsafe
	{
		crate::logln!("jiffies = {}", crate::time::JIFFIES);
	}
}

fn yesss()
{
	crate::serial_println!("###################%%%%%%%%%####################%%%%%%%%%%%%###((//*        ");
	crate::serial_println!("###################%%%%%%%%######################%%%%%%%%%%%####((/**,      ");
	crate::serial_println!("####################%%%%%%%######################%%%%%%%%%%%%####((/*,      ");
	crate::serial_println!("#####################%%%%%%######################%%%%%%%%%%%%%####((/*      ");
	crate::serial_println!("#####################%%%%%%######################%%%%%%%%%%%%%%####((*      ");
	crate::serial_println!("#####################%%%%%%%#####################%%%%%%%%%%%%%%%###((/,     ");
	crate::serial_println!("#####################%%%%%%%######################%%%%%%%%%%%%%####(((*     ");
	crate::logln!("######################%%%&%%######################%%%%%%%%%%%%###(/****,    ");
	crate::logln!("########((############%%%%&%#######(##############%%%%%%%%%%%##(/,....**    ");
	crate::logln!("######(*,..,*/((((######%%%%%####((((((############%%%%%%%%###(,  ....,,    ");
	crate::logln!("####(*/(((//*,. .,/(((###%%%%#####((((###############%%%%%###/  .,,,,,,*    ");
	crate::logln!("###((####((((///*.  .*((###%%####(((((####################(, .,**,****,**   ");
	crate::logln!("#%%##%%%#((((((((//*. .,/(#######(((((##############(/*,.,*/(##(/////////,  ");
	crate::logln!("#%%%%%%%%%%%(((((#((/*,...,/(###(((((((#####%%####(////(((((,   **//((##(,  ");
	crate::logln!("#%%%%&&%%%%%(/(*  ,#%#///**,...,,,**/(((#####%%%%(/(#((((##(//((//((#####/* ");
	crate::logln!("####%%%&&&%%%%#(.    .##((///////////(((####((((#%%((##########((((#######, ");
	crate::logln!("#######%%%%&%%%%%%##(**(##((###############((((((#%#(##################(##* ");
	crate::logln!("############%%&%%%%%%%%%%%%&&&&&&%#########(((((((##/(###############(((((* ");
	crate::logln!("###############%%%&&&%%%%%%%%%%#############((((((#%#(#############((((((/* ");
	crate::logln!("####################%%%%%%%#################((((((###((############(((((//* ");
	crate::logln!("((#((((#####################################(((((((#%((##########(((((////, ");
	crate::logln!("(((((((((((((###############################((((((((#%((########(((((////,, ");
	crate::logln!("((((((((((((((((((##########################((((((#(#%#(######((((((///***  ");
	crate::logln!("(((((((((((((((((((((((#(###################((((###((#%((##(((((((/////***  ");
	crate::logln!("((((((((((((((((((((((((((((((((((##################((%%/(#((((((////***.   ");
	crate::logln!("/(((((((((((((((((((((((((((((((((((#################(#%#/(((((//////**,    ");
	crate::logln!("/((((((((((((((((((((((((((((((((((((###################%(/(////////***.    ");
	crate::logln!("/(((((((((((((((((((((((((((((((((((((#%%%%#########(#(##%////////****.     ");
	crate::logln!("//((((((((((((((((((((((((((((((((((/(%#((#####((((((((###(///////***,.     ");
	crate::logln!("//((((((((((((((((((((((((((((((((//##(///////(#%%#(((((((/*/////****,      ");
	crate::logln!("//(((((((((((((((((((((((((((((((///*,,,,,,,,,.....  ....,*//////***,.      ");
	crate::logln!("///((((((((((((((((((((((((((((((//******,,,,..   .,*///////////*****       ");
	crate::serial_println!("//((((((((((((((((((((((((((((((//********,,,,,,,**/////////////****.       ");
	crate::serial_println!("///(((((((((((((((((((((((((((((//***********,,*/(((((((((/////****.        ");
	crate::serial_println!("///(((((((((((((((((((((((((((((//***********,*/((((((((((////****.         ");
	crate::serial_println!("////((((((((((((((((((((((((((((/***********,,/(###((((((////*****          ");
	crate::serial_println!("/////(((((((((((((((((((((((((((//*******,,,*/(####((((((//******.          ");
	crate::serial_println!("/////(((((((((((((((((((((((((((((//****,,,*/#%%##(/,,**********,           ");
	crate::serial_println!("//////((((((((((((((((//*****///(#((//**,*/#%%##(..//,,,*******,            ");
	crate::serial_println!("**/////((((((((((((((((///*****(/*,*(((//(###(*,(%*.,*/(//****,             ");
	crate::serial_println!("***//////(((((((((((((((((((/*,,,*((/*,,***,,*#(. ,/((((//***,.             ");
	crate::serial_println!("*****///////(((((((((((((((((/*,,,,,,*/(((//,.  ,(###((//***,               ");
	crate::serial_println!("******////////((((((((((((####(//*,,,,.... .,*(######(//**,,*               ");
	crate::serial_println!("********////////(((((((((#########(////(((###%%%####(//**,.                 ");
	crate::serial_println!("**********///////(((((((((##############%%%%%%%####(//**,                   ");
	crate::serial_println!("***********////////((((((((#############%%%%%%####((/*,.                    ");
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
