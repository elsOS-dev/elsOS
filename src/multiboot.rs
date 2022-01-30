use crate::print;
use crate::println;
use crate::boot;
use crate::utilities;

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
			utilities::from_c_str((&self.str_ptr as *const _) as *const u8)
		}
	}
}

pub fn check_magic(magic: u32) -> bool
{
	let magic_ok = magic == BOOTLOADER_MAGIC;

	print!("[{}] multiboot2 magic number", boot::ok_fail(magic_ok));
	if !magic_ok
	{
		print!(": expected {:#0x}, got {:#0x}.", BOOTLOADER_MAGIC, magic);
	}
	print!("\n");

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

pub fn parse(address: u32) -> bool
{
	let alignment_ok = address & 7 == 0;

	println!("[{}] multiboot2 structure address alignment", boot::ok_fail(alignment_ok));


	unsafe
	{
		let info_header = address as *const MultibootInfoHeader;
		println!("[INFO] multiboot2 information structure total size: {}", (*info_header).total_size);

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
					print!("[INFO] {}: ", if (*tag).tag_type == 1 { "cmd line" } else { "bootloader" });
					while string[i] != b'\0'
					{
						print!("{}", string[i] as char);
						i += 1;
					}
					print!("\n");
				},
				MULTIBOOT_TAG_TYPE_END => {
					println!("[INFO] end of multiboot2 information structure");
					break
				},
				_ => {}//println!("found tag of type {} and size {}", type_name((*tag).tag_type), (*tag).size)
			};

			address += ((*tag).size + 7) & !7;
		}
	}

	true
}
