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
					},
					"int" =>
					{
						let n = i32::from_str_radix(arg, 16).unwrap_or_else(|_| { -1 });
						if n >= 0x00 && n <= 0xff
						{
							int(n as u8);
						}
					},
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

// dirty interrupt testing
fn int(n: u8)
{
	use core::arch::asm;
	unsafe
	{
		if n == 0 {asm!("int 0");}
		else if n == 1 {asm!("int 1");}
		else if n == 2 {asm!("int 2");}
		else if n == 3 {asm!("int 3");}
		else if n == 4 {asm!("int 4");}
		else if n == 5 {asm!("int 5");}
		else if n == 6 {asm!("int 6");}
		else if n == 7 {asm!("int 7");}
		else if n == 8 {asm!("int 8");}
		else if n == 9 {asm!("int 9");}
		else if n == 10 {asm!("int 10");}
		else if n == 11 {asm!("int 11");}
		else if n == 12 {asm!("int 12");}
		else if n == 13 {asm!("int 13");}
		else if n == 14 {asm!("int 14");}
		else if n == 15 {asm!("int 15");}
		else if n == 16 {asm!("int 16");}
		else if n == 17 {asm!("int 17");}
		else if n == 18 {asm!("int 18");}
		else if n == 19 {asm!("int 19");}
		else if n == 20 {asm!("int 20");}
		else if n == 21 {asm!("int 21");}
		else if n == 22 {asm!("int 22");}
		else if n == 23 {asm!("int 23");}
		else if n == 24 {asm!("int 24");}
		else if n == 25 {asm!("int 25");}
		else if n == 26 {asm!("int 26");}
		else if n == 27 {asm!("int 27");}
		else if n == 28 {asm!("int 28");}
		else if n == 29 {asm!("int 29");}
		else if n == 30 {asm!("int 30");}
		else if n == 31 {asm!("int 31");}
		else if n == 32 {asm!("int 32");}
		else if n == 33 {asm!("int 33");}
		else if n == 34 {asm!("int 34");}
		else if n == 35 {asm!("int 35");}
		else if n == 36 {asm!("int 36");}
		else if n == 37 {asm!("int 37");}
		else if n == 38 {asm!("int 38");}
		else if n == 39 {asm!("int 39");}
		else if n == 40 {asm!("int 40");}
		else if n == 41 {asm!("int 41");}
		else if n == 42 {asm!("int 42");}
		else if n == 43 {asm!("int 43");}
		else if n == 44 {asm!("int 44");}
		else if n == 45 {asm!("int 45");}
		else if n == 46 {asm!("int 46");}
		else if n == 47 {asm!("int 47");}
		else if n == 48 {asm!("int 48");}
		else if n == 49 {asm!("int 49");}
		else if n == 50 {asm!("int 50");}
		else if n == 51 {asm!("int 51");}
		else if n == 52 {asm!("int 52");}
		else if n == 53 {asm!("int 53");}
		else if n == 54 {asm!("int 54");}
		else if n == 55 {asm!("int 55");}
		else if n == 56 {asm!("int 56");}
		else if n == 57 {asm!("int 57");}
		else if n == 58 {asm!("int 58");}
		else if n == 59 {asm!("int 59");}
		else if n == 60 {asm!("int 60");}
		else if n == 61 {asm!("int 61");}
		else if n == 62 {asm!("int 62");}
		else if n == 63 {asm!("int 63");}
		else if n == 64 {asm!("int 64");}
		else if n == 65 {asm!("int 65");}
		else if n == 66 {asm!("int 66");}
		else if n == 67 {asm!("int 67");}
		else if n == 68 {asm!("int 68");}
		else if n == 69 {asm!("int 69");}
		else if n == 70 {asm!("int 70");}
		else if n == 71 {asm!("int 71");}
		else if n == 72 {asm!("int 72");}
		else if n == 73 {asm!("int 73");}
		else if n == 74 {asm!("int 74");}
		else if n == 75 {asm!("int 75");}
		else if n == 76 {asm!("int 76");}
		else if n == 77 {asm!("int 77");}
		else if n == 78 {asm!("int 78");}
		else if n == 79 {asm!("int 79");}
		else if n == 80 {asm!("int 80");}
		else if n == 81 {asm!("int 81");}
		else if n == 82 {asm!("int 82");}
		else if n == 83 {asm!("int 83");}
		else if n == 84 {asm!("int 84");}
		else if n == 85 {asm!("int 85");}
		else if n == 86 {asm!("int 86");}
		else if n == 87 {asm!("int 87");}
		else if n == 88 {asm!("int 88");}
		else if n == 89 {asm!("int 89");}
		else if n == 90 {asm!("int 90");}
		else if n == 91 {asm!("int 91");}
		else if n == 92 {asm!("int 92");}
		else if n == 93 {asm!("int 93");}
		else if n == 94 {asm!("int 94");}
		else if n == 95 {asm!("int 95");}
		else if n == 96 {asm!("int 96");}
		else if n == 97 {asm!("int 97");}
		else if n == 98 {asm!("int 98");}
		else if n == 99 {asm!("int 99");}
		else if n == 100 {asm!("int 100");}
		else if n == 101 {asm!("int 101");}
		else if n == 102 {asm!("int 102");}
		else if n == 103 {asm!("int 103");}
		else if n == 104 {asm!("int 104");}
		else if n == 105 {asm!("int 105");}
		else if n == 106 {asm!("int 106");}
		else if n == 107 {asm!("int 107");}
		else if n == 108 {asm!("int 108");}
		else if n == 109 {asm!("int 109");}
		else if n == 110 {asm!("int 110");}
		else if n == 111 {asm!("int 111");}
		else if n == 112 {asm!("int 112");}
		else if n == 113 {asm!("int 113");}
		else if n == 114 {asm!("int 114");}
		else if n == 115 {asm!("int 115");}
		else if n == 116 {asm!("int 116");}
		else if n == 117 {asm!("int 117");}
		else if n == 118 {asm!("int 118");}
		else if n == 119 {asm!("int 119");}
		else if n == 120 {asm!("int 120");}
		else if n == 121 {asm!("int 121");}
		else if n == 122 {asm!("int 122");}
		else if n == 123 {asm!("int 123");}
		else if n == 124 {asm!("int 124");}
		else if n == 125 {asm!("int 125");}
		else if n == 126 {asm!("int 126");}
		else if n == 127 {asm!("int 127");}
		else if n == 128 {asm!("int 128");}
		else if n == 129 {asm!("int 129");}
		else if n == 130 {asm!("int 130");}
		else if n == 131 {asm!("int 131");}
		else if n == 132 {asm!("int 132");}
		else if n == 133 {asm!("int 133");}
		else if n == 134 {asm!("int 134");}
		else if n == 135 {asm!("int 135");}
		else if n == 136 {asm!("int 136");}
		else if n == 137 {asm!("int 137");}
		else if n == 138 {asm!("int 138");}
		else if n == 139 {asm!("int 139");}
		else if n == 140 {asm!("int 140");}
		else if n == 141 {asm!("int 141");}
		else if n == 142 {asm!("int 142");}
		else if n == 143 {asm!("int 143");}
		else if n == 144 {asm!("int 144");}
		else if n == 145 {asm!("int 145");}
		else if n == 146 {asm!("int 146");}
		else if n == 147 {asm!("int 147");}
		else if n == 148 {asm!("int 148");}
		else if n == 149 {asm!("int 149");}
		else if n == 150 {asm!("int 150");}
		else if n == 151 {asm!("int 151");}
		else if n == 152 {asm!("int 152");}
		else if n == 153 {asm!("int 153");}
		else if n == 154 {asm!("int 154");}
		else if n == 155 {asm!("int 155");}
		else if n == 156 {asm!("int 156");}
		else if n == 157 {asm!("int 157");}
		else if n == 158 {asm!("int 158");}
		else if n == 159 {asm!("int 159");}
		else if n == 160 {asm!("int 160");}
		else if n == 161 {asm!("int 161");}
		else if n == 162 {asm!("int 162");}
		else if n == 163 {asm!("int 163");}
		else if n == 164 {asm!("int 164");}
		else if n == 165 {asm!("int 165");}
		else if n == 166 {asm!("int 166");}
		else if n == 167 {asm!("int 167");}
		else if n == 168 {asm!("int 168");}
		else if n == 169 {asm!("int 169");}
		else if n == 170 {asm!("int 170");}
		else if n == 171 {asm!("int 171");}
		else if n == 172 {asm!("int 172");}
		else if n == 173 {asm!("int 173");}
		else if n == 174 {asm!("int 174");}
		else if n == 175 {asm!("int 175");}
		else if n == 176 {asm!("int 176");}
		else if n == 177 {asm!("int 177");}
		else if n == 178 {asm!("int 178");}
		else if n == 179 {asm!("int 179");}
		else if n == 180 {asm!("int 180");}
		else if n == 181 {asm!("int 181");}
		else if n == 182 {asm!("int 182");}
		else if n == 183 {asm!("int 183");}
		else if n == 184 {asm!("int 184");}
		else if n == 185 {asm!("int 185");}
		else if n == 186 {asm!("int 186");}
		else if n == 187 {asm!("int 187");}
		else if n == 188 {asm!("int 188");}
		else if n == 189 {asm!("int 189");}
		else if n == 190 {asm!("int 190");}
		else if n == 191 {asm!("int 191");}
		else if n == 192 {asm!("int 192");}
		else if n == 193 {asm!("int 193");}
		else if n == 194 {asm!("int 194");}
		else if n == 195 {asm!("int 195");}
		else if n == 196 {asm!("int 196");}
		else if n == 197 {asm!("int 197");}
		else if n == 198 {asm!("int 198");}
		else if n == 199 {asm!("int 199");}
		else if n == 200 {asm!("int 200");}
		else if n == 201 {asm!("int 201");}
		else if n == 202 {asm!("int 202");}
		else if n == 203 {asm!("int 203");}
		else if n == 204 {asm!("int 204");}
		else if n == 205 {asm!("int 205");}
		else if n == 206 {asm!("int 206");}
		else if n == 207 {asm!("int 207");}
		else if n == 208 {asm!("int 208");}
		else if n == 209 {asm!("int 209");}
		else if n == 210 {asm!("int 210");}
		else if n == 211 {asm!("int 211");}
		else if n == 212 {asm!("int 212");}
		else if n == 213 {asm!("int 213");}
		else if n == 214 {asm!("int 214");}
		else if n == 215 {asm!("int 215");}
		else if n == 216 {asm!("int 216");}
		else if n == 217 {asm!("int 217");}
		else if n == 218 {asm!("int 218");}
		else if n == 219 {asm!("int 219");}
		else if n == 220 {asm!("int 220");}
		else if n == 221 {asm!("int 221");}
		else if n == 222 {asm!("int 222");}
		else if n == 223 {asm!("int 223");}
		else if n == 224 {asm!("int 224");}
		else if n == 225 {asm!("int 225");}
		else if n == 226 {asm!("int 226");}
		else if n == 227 {asm!("int 227");}
		else if n == 228 {asm!("int 228");}
		else if n == 229 {asm!("int 229");}
		else if n == 230 {asm!("int 230");}
		else if n == 231 {asm!("int 231");}
		else if n == 232 {asm!("int 232");}
		else if n == 233 {asm!("int 233");}
		else if n == 234 {asm!("int 234");}
		else if n == 235 {asm!("int 235");}
		else if n == 236 {asm!("int 236");}
		else if n == 237 {asm!("int 237");}
		else if n == 238 {asm!("int 238");}
		else if n == 239 {asm!("int 239");}
		else if n == 240 {asm!("int 240");}
		else if n == 241 {asm!("int 241");}
		else if n == 242 {asm!("int 242");}
		else if n == 243 {asm!("int 243");}
		else if n == 244 {asm!("int 244");}
		else if n == 245 {asm!("int 245");}
		else if n == 246 {asm!("int 246");}
		else if n == 247 {asm!("int 247");}
		else if n == 248 {asm!("int 248");}
		else if n == 249 {asm!("int 249");}
		else if n == 250 {asm!("int 250");}
		else if n == 251 {asm!("int 251");}
		else if n == 252 {asm!("int 252");}
		else if n == 253 {asm!("int 253");}
		else if n == 254 {asm!("int 254");}
		else if n == 255 {asm!("int 255");}
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
