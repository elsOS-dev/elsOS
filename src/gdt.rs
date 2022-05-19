// using https://wiki.osdev.org/Global_Descriptor_Table#Segment_Descriptor to fill the values

use core::ffi::c_void;
use core::mem::size_of;
use crate::libc;

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


// Base bit //
// We choose to begin each segment at 0, so each segments covers the entire
// memory.

// Access bit //
// First bit, is present bit, should be always 1 except for the null entry

// Second and third bit, is the privilege level, 00 means highest privilege, 
// so every kernel segment has 00, 11 is the lowest privilege level, so every
// user segment has 11.

// fourth bit is defines if the segment is a system segment or a code/data 
// segment, we don't have any system segments yet so it should be 1 on each 
// segment.

// The fifth bit, is the executable bit. If it's a code segment that should be
// executable it has to bet set to 1 like in kernel and user code segments.

// The sixth bit, depends on the type of segment:
// for data segments, it defines if the data grows up or down like in kernel 
// and user stack. Because the stack grows down.
// for code segments, 0 means the code can only be executed from the same ring.
// and 1 means it can be executed by equal or lower ring levels.
// we choose 0 because it is likely the more secure option.

// The seventh is the R/W bit:
// for code segments it's a readable bit. if 0 read is not allowed, 
// write is never allowed.
// for data segments: writable bit: if 0, write is not allowed, read is always
// allowed.
// We choose or kernel segments and user segments to be readable and writable,
// because we want them to be more permissive, and in the future tweak them if
// needed in a more restrictive way.

// The last bit must be set to 0 it is set to 1 when the CPU accesses the 
// segment


// Flags and Limits //
// Flags are composed with 4 bits,
// c means 1100
// first bit: Granularity flag: if 0 the blocks are 1 byte, if 1 4 kbytes
// second bit: processor mode, 0 16 bit, 1: 32 bit protected
// third bit: if set to 1, defines 64 bit
// fourth bit: reserved
//  we are in 32 bit protected using the 4kbytes blocks, this makes 1100, or C
// in hexadecimal.
// 5-8: last bit of the 5 bit limits value, makes for a total of 20 bits:
// A 20-bit value, tells the maximum addressable unit, either in 1 byte
// units, or in 4KiB pages. Hence, if you choose page granularity and set the
// Limit value to 0xFFFFF the segment will span the full 4 GiB address space
//  in 32-bit mode. 


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
		libc::memcpy(0x800 as *mut c_void, (&GDT_TABLE as *const _) as *const c_void, size);
		_gp.limit = size as u16;
		_gdt_flush();
	}
}

