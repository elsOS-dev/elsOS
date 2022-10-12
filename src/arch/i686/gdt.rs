use core::mem::size_of;

#[repr(C, packed)]
struct gdt_descriptor
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

static GDT_DESCRIPTOR: gdt_descriptor = gdt_descriptor
{
	limit: size_of::<gdt>() as u16 - 1,
	base: 0x800
};

#[link_section = ".gdt"]
static GDT: gdt = gdt
{
	null: gdt_entry
	{
		limit0: 0,
		base0: 0,
		base1: 0,
		access_byte: 0,
		limit1_flags: 0,
		base2: 0
	},
	kernel_code: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1001_1010,
		limit1_flags: 0b1100_1111,
		base2: 0
	},
	kernel_data: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1001_0010,
		limit1_flags: 0b1100_1111,
		base2: 0
	},
	kernel_stack: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1001_0110,
		limit1_flags: 0b1100_1111,
		base2: 0
	},
	user_code: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1111_1010,
		limit1_flags: 0b1100_1111,
		base2: 0
	},
	user_data: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1111_0010,
		limit1_flags: 0b1100_1111,
		base2: 0
	},
	user_stack: gdt_entry
	{
		limit0: 0xffff,
		base0: 0,
		base1: 0,
		access_byte: 0b1111_0110,
		limit1_flags: 0b1100_1111,
		base2: 0
	},
};

extern "C"
{
	fn _gdt_flush(gdt_descriptor: &gdt_descriptor);
}

pub fn init()
{
	unsafe
	{
		_gdt_flush(&GDT_DESCRIPTOR);
	}
}

/***************************************************************************************************
* documentation (for more details: https://wiki.osdev.org/Global_Descriptor_Table)
*
*  base gdt entry:
*  |                              1|1                               3|
*  |0                             5|6                               1|
*  |-------------------------------|---------------------------------|
*  | [16] limit low  always 0xffff | [16] base low          always 0 |
*  |1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1|0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0|
*  |           dw 0xffff           |               dw 0              |
*  |-------------------------------|---------------------------------|
*  |3             3|4             4|1     1|2     2|2               3|
*  |1             9|0             8|6     9|0     3|4               2|
*  |---------------|---------------|-------|-------|-----------------|
*  | [8] base    0 | [8] access    | lim h | flags | [8] base        |
*  |               |          D D  |       |r      |                 |
*  |               |A R D     P P P|       |e   D  |                 |
*  |0 0 0 0 0 0 0 0|0 W C E S L L 1|1 1 1 1|0 L B G|0 0 0 0 0 0 0 0 0|
*  |     db 0      | db 1******0b  | db 11001111b  |      db 0       |
*  |---------------|---------------|-------|-------|-----------------|
*
*  this representation is reverted for endianness.
*
*  for elsOS, we will use a protected mode, with flat memory model for paging
*  this means that we will set the base to 0 and limit to 0xffff_f (maximum)
*  this will cut into limit low with 0xffff and an f in {limit high + flags}
*
*
*  after this, we have to fill the access byte:
*   P  : Present bit. Allows an entry to refer to a valid segment. Must be 1.
*   DPL: Descriptor Privilege Level. CPU Privilege level of the segment.
*   DPL: 0|00 = highest privilege (kernel), 3|11 = lowest privilege (user applications).
*   S  : Descriptor type bit. System segment if 0 (TSS..), code or data segment if 1.
*
*   E  : Executable bit. Data segment if 0, executable code segment if 1.
*   DC : For data selectors, direction bit. For code selector, conforming bit.
*   RW : For code segment, read access (write never allowed). For data segment, write access.
*   A  : Accessed bit. 0 by default, the CPU will set it when the segment is accessed.
*
*   Direction bit: 0 if the segment grows up, 1 if the segment grows down (offset > limit).
*   Conforming bit: If 0, code can only be executed from ring == DPL. If 1, from ring <= DPL.
*
*
*  for the flags:
*   G  1: Granularity. 0 if byte scale, 1 if 4KiB page scale. (byte granularity vs page granularity).
*   DB 1: Size flag. 16-bit protected mode segment if 0. 32-bit protected mode segment if 1.
*   L  0: Long mode. 1 if this defines a 64-bit segment.
*
*
*  by default, this will give us:
*	{
*		limit0: 0xffff,
*		base0: 0,
*		base1: 0,
*		access_byte: 0b1***_***0,
*		limit1_flags: 0b1100_1111,
*		base2: 0
*	},
***************************************************************************************************/

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
