pub mod pageframe;

use core::ffi::c_void;
use crate::multiboot::MultibootTagMmap;

extern "C"
{
	static _kernel_start: c_void;
	static _kernel_end: c_void;
}

pub fn get_mem_size(mmap: *const MultibootTagMmap, mmap_size: usize) -> u64
{
	static mut MEM_SIZE_BYTES: u64 = 0;
	unsafe
	{
		if MEM_SIZE_BYTES > 0
		{
			return MEM_SIZE_BYTES;
		}
		// crate::logln!("\x1B[33mmmap: {:#x?}\x1B[39m", (*mmap).entries(mmap_size));
		for mmap_entry in (*mmap).entries(mmap_size)
		{
			MEM_SIZE_BYTES += mmap_entry.len as u64;
		}
		if MEM_SIZE_BYTES > usize::MAX as u64
		{
			panic!("This version of ElsOS is in 32 bit, it only supports {}Mo of RAM, you have {}Mo installed", (usize::MAX / 1024) / 1024, (MEM_SIZE_BYTES / 1024) / 1024);
		}
		return MEM_SIZE_BYTES;
	}
}

pub fn get_kernel_size() -> usize
{
	let kernel_start: usize;
	let kernel_end: usize;
	unsafe
	{
		kernel_start = &_kernel_start as *const _ as usize;
		kernel_end =  &_kernel_end as *const _ as usize;
	}
	let kernel_size = kernel_end - kernel_start;
	crate::logln!("{:#x?} {:#x?}", kernel_start, kernel_end);
	return kernel_size;
}
