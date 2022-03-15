use crate::log;
use crate::logln;
use crate::ferramenta;
use crate::ok_fail;
use core::slice;
use core::mem::size_of;

pub static mut MULTIBOOT_MMAP: *const MultibootTagMmap = core::ptr::null();
pub static mut MULTIBOOT_MMAP_ENTRIES: usize = 0;

const BOOTLOADER_MAGIC: u32 = 0x36d76289;

const MULTIBOOT_TAG_TYPE_END: u32				= 0;
const MULTIBOOT_TAG_TYPE_CMDLINE: u32			= 1;
const MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME: u32	= 2;
const MULTIBOOT_TAG_TYPE_MODULE: u32			= 3;
const MULTIBOOT_TAG_TYPE_BASIC_MEMINFO: u32		= 4;
const MULTIBOOT_TAG_TYPE_BOOTDEV: u32			= 5;
const MULTIBOOT_TAG_TYPE_MMAP: u32				= 6;
const MULTIBOOT_TAG_TYPE_VBE: u32				= 7;
const MULTIBOOT_TAG_TYPE_FRAMEBUFFER: u32		= 8;
const MULTIBOOT_TAG_TYPE_ELF_SECTIONS: u32		= 9;
const MULTIBOOT_TAG_TYPE_APM: u32				= 10;
const MULTIBOOT_TAG_TYPE_EFI32: u32				= 11;
const MULTIBOOT_TAG_TYPE_EFI64: u32				= 12;
const MULTIBOOT_TAG_TYPE_SMBIOS: u32			= 13;
const MULTIBOOT_TAG_TYPE_ACPI_OLD: u32			= 14;
const MULTIBOOT_TAG_TYPE_ACPI_NEW: u32			= 15;
const MULTIBOOT_TAG_TYPE_NETWORK: u32			= 16;
const MULTIBOOT_TAG_TYPE_EFI_MMAP: u32			= 17;
const MULTIBOOT_TAG_TYPE_EFI_BS: u32			= 18;
const MULTIBOOT_TAG_TYPE_EFI32_IH: u32			= 19;
const MULTIBOOT_TAG_TYPE_EFI64_IH: u32			= 20;
const MULTIBOOT_TAG_TYPE_LOAD_BASE_ADDR: u32	= 21;

#[repr(C)]
struct MultibootInfoHeader
{
	total_size: u32,
	reserved: u32
}

#[repr(C)]
struct MultibootTag
{
	tag_type: u32,
	size: u32
}

#[repr(C)]
struct MultibootTagString
{
	tag_type: u32,
	size: u32,
	str_ptr: &'static [u8]
}

impl MultibootTagString
{
	fn string(&self) -> &'static [u8]
	{
		unsafe
		{
			ferramenta::from_c_str((&self.str_ptr as *const _) as *const u8)
		}
	}
}

struct MultibootTagBasicMeminfo
{
	tag_type: u32,
	size: u32,
	mem_lower: u32,
	mem_upper: u32
}

// constants for the type inside MultiBootMmapEntry struct
const MULTIBOOT_MEMORY_AVAILABLE: u32 = 1;
const MULTIBOOT_MEMORY_RESERVED: u32 = 2;
const MULTIBOOT_MEMORY_ACPI_RECLAIMABLE: u32 = 3;
const MULTIBOOT_MEMORY_NVS: u32 = 4;
const MULTIBOOT_MEMORY_BADRAM: u32 = 5;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MultibootMmapEntry
{
	pub addr: u32,
	addr_upper: u32,
	pub len: u32,
	len_upper: u32,
	pub tag_type: u32,
	zero: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct MultibootTagMmap
{

	tag_type: u32,
	size: u32,
	entry_size: u32,
	entry_version: u32,
	entries_ptr: &'static [MultibootMmapEntry]//u32, // struct *MultibootMmapEntry
}

impl MultibootTagMmap
{
	pub fn entries(&self, number: usize) -> &'static [MultibootMmapEntry]
	{
		unsafe
		{
			slice::from_raw_parts((&self.entries_ptr as *const _) as *const MultibootMmapEntry, number)
		}
	}
}

pub fn check_magic(magic: u32) -> bool
{
	let magic_ok = magic == BOOTLOADER_MAGIC;

	log!("[{}] multiboot2 magic number", ok_fail(magic_ok));
	if !magic_ok
	{
		log!(": expected {:#0x}, got {:#0x}.", BOOTLOADER_MAGIC, magic);
	}
	logln!();

	magic_ok
}

fn type_name(tag_type: u32) -> &'static str
{
	match tag_type
	{
		MULTIBOOT_TAG_TYPE_END => "MULTIBOOT_TAG_TYPE_END",
		MULTIBOOT_TAG_TYPE_CMDLINE => "MULTIBOOT_TAG_TYPE_CMDLINE",
		MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME => "MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME",
		MULTIBOOT_TAG_TYPE_MODULE => "MULTIBOOT_TAG_TYPE_MODULE",
		MULTIBOOT_TAG_TYPE_BASIC_MEMINFO => "MULTIBOOT_TAG_TYPE_BASIC_MEMINFO",
		MULTIBOOT_TAG_TYPE_BOOTDEV => "MULTIBOOT_TAG_TYPE_BOOTDEV",
		MULTIBOOT_TAG_TYPE_MMAP => "MULTIBOOT_TAG_TYPE_MMAP",
		MULTIBOOT_TAG_TYPE_VBE => "MULTIBOOT_TAG_TYPE_VBE",
		MULTIBOOT_TAG_TYPE_FRAMEBUFFER => "MULTIBOOT_TAG_TYPE_FRAMEBUFFER",
		MULTIBOOT_TAG_TYPE_ELF_SECTIONS => "MULTIBOOT_TAG_TYPE_ELF_SECTIONS",
		MULTIBOOT_TAG_TYPE_APM => "MULTIBOOT_TAG_TYPE_APM",
		MULTIBOOT_TAG_TYPE_EFI32 => "MULTIBOOT_TAG_TYPE_EFI32",
		MULTIBOOT_TAG_TYPE_EFI64 => "MULTIBOOT_TAG_TYPE_EFI64",
		MULTIBOOT_TAG_TYPE_SMBIOS => "MULTIBOOT_TAG_TYPE_SMBIOS",
		MULTIBOOT_TAG_TYPE_ACPI_OLD => "MULTIBOOT_TAG_TYPE_ACPI_OLD",
		MULTIBOOT_TAG_TYPE_ACPI_NEW => "MULTIBOOT_TAG_TYPE_ACPI_NEW",
		MULTIBOOT_TAG_TYPE_NETWORK => "MULTIBOOT_TAG_TYPE_NETWORK",
		MULTIBOOT_TAG_TYPE_EFI_MMAP => "MULTIBOOT_TAG_TYPE_EFI_MMAP",
		MULTIBOOT_TAG_TYPE_EFI_BS => "MULTIBOOT_TAG_TYPE_EFI_BS",
		MULTIBOOT_TAG_TYPE_EFI32_IH => "MULTIBOOT_TAG_TYPE_EFI32_IH",
		MULTIBOOT_TAG_TYPE_EFI64_IH => "MULTIBOOT_TAG_TYPE_EFI64_IH",
		MULTIBOOT_TAG_TYPE_LOAD_BASE_ADDR => "MULTIBOOT_TAG_TYPE_LOAD_BASE_ADDR",
		_ => "unknown"
	}
}

fn handle_qwerty()
{
	unsafe
	{
		crate::SETTINGS.layout = 1;
	}
}

fn handle_serial()
{
	unsafe
	{
		crate::SETTINGS.has_serial = true;
	}
}

fn parse_args(args: &[u8])
{
	let mut previous_index: usize = 0;
	for (i, chr) in args.iter().enumerate()
	{
		let i = if i + 1 == args.len() { i + 1 } else { i };

		if *chr == b' ' || i == args.len()
		{
			let arg = core::str::from_utf8(&args[previous_index..i]).unwrap_or_else(|_| { "" });

			logln!("[INFO] paring arg {}", arg);

			match arg
			{
				"qwerty" => handle_qwerty(),
				"serial" => handle_serial(),
				_ => {}
			};

			previous_index = i + 1;
		}
	}
}

pub fn parse(address: u32) -> bool
{
	let alignment_ok = address & 7 == 0;

	logln!("[{}] multiboot2 structure address alignment", ok_fail(alignment_ok));

	unsafe
	{
		let info_header = address as *const MultibootInfoHeader;
		logln!("[INFO] multiboot2 information structure total size: {}", (*info_header).total_size);

		let mut address = address + 8;
		loop
		{
			let tag = address as *const MultibootTag;

			//print!("{:#0x}: ", address);
			match (*tag).tag_type
			{
				MULTIBOOT_TAG_TYPE_CMDLINE | MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME => {
					let tag = tag as *const MultibootTagString;
					let mut i = 0;

					let string = (*tag).string();
					log!("[INFO] {}: ", if (*tag).tag_type == MULTIBOOT_TAG_TYPE_CMDLINE { "cmd line" } else { "bootloader" });
					while string[i] != b'\0'
					{
						log!("{}", string[i] as char);
						i += 1;
					}
					logln!();
					if (*tag).tag_type == MULTIBOOT_TAG_TYPE_CMDLINE
					{
						parse_args(&string[..i]);
					}
				},
				MULTIBOOT_TAG_TYPE_END => {
					logln!("[INFO] end of multiboot2 information structure");
					break
				},
				MULTIBOOT_TAG_TYPE_MMAP =>
				{
					MULTIBOOT_MMAP = tag as *const MultibootTagMmap;
					MULTIBOOT_MMAP_ENTRIES = (*tag).size as usize / size_of::<MultibootMmapEntry>();
				}
				_ => {}//crate::println!("found tag of type {} and size {}", type_name((*tag).tag_type), (*tag).size)
			};

			address += ferramenta::align((*tag).size as usize, 8) as u32;
		}
	}

	true
}
