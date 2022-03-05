// using https://wiki.osdev.org/Global_Descriptor_Table#Segment_Descriptor to fill the values

use core::mem::size_of;

#[repr(C, packed)]
struct gdt_ptr
{
	limit: u16,
	base: u32
}

#[repr(C, packed)]
struct gdt_entry
{
	limit0: u16,
	base0: u16,
	base1: u8,
	access_byte: u8,
	limit1_flags: u8,
	base2: u8
}

#[repr(C, packed)]
struct gdt
{
	null: gdt_entry,
	kernel_code: gdt_entry,
	kernel_data: gdt_entry,
	kernel_stack: gdt_entry,
	user_code: gdt_entry,
	user_data: gdt_entry,
	user_stack: gdt_entry,
}

static mut GDT_TABLE: gdt = gdt
{
	null: gdt_entry
	{
		limit0: 0,
		base0: 0,
		base1: 0,
		access_byte: 0x00,
		limit1_flags: 0x00,
		base2: 0
	},
	kernel_code: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1001_1010,
		limit1_flags: 0xcf,
		base2: 0
	},
	kernel_data: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1001_0010,
		limit1_flags: 0xcf,
		base2: 0
	},
	kernel_stack: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1001_0110,
		limit1_flags: 0xcf,
		base2: 0
	},
	user_code: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1111_1010,
		limit1_flags: 0xcf,
		base2: 0
	},
	user_data: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1111_0010,
		limit1_flags: 0xcf,
		base2: 0
	},
	user_stack: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1111_0110,
		limit1_flags: 0xcf,
		base2: 0
	},
};

extern "C"
{
	fn _gdt_flush();
	fn memcpy(dst: *const u8, src: *const u8, size: usize);
}

#[no_mangle]
static mut _gp: gdt_ptr = gdt_ptr
{
	limit: 0,
	base: 0x800
};

pub fn init_gdt()
{
	let size = size_of::<gdt>() - 1;
	unsafe
	{
		memcpy(0x800 as *const u8, (&GDT_TABLE as *const _) as *const u8, size);
		_gp.limit = size as u16;
		_gdt_flush();
	}
}

