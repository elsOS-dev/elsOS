pub mod pageframe;
mod pagedirectory;
mod pagetable;

use core::ffi::c_void;
use crate::multiboot::MultibootTagMmap;

extern "C"
{
	static _kernel_start: c_void;
	static _kernel_end: c_void;
}

pub fn get_mem_size(mmap: *const MultibootTagMmap, mmap_size: usize) -> usize
{
	static mut MEM_SIZE_BYTES: u64 = 0;
	unsafe
	{
		if MEM_SIZE_BYTES > 0
		{
			return MEM_SIZE_BYTES as usize;
		}
		crate::logln!("\x1B[33mmmap: {:#x?}\x1B[39m", (*mmap).entries(mmap_size));
		for mmap_entry in (*mmap).entries(mmap_size)
		{
			MEM_SIZE_BYTES += mmap_entry.len as u64;
		}
		if MEM_SIZE_BYTES > usize::MAX as u64
		{
			panic!("This version of ElsOS is in 32 bit, it only supports {}Mo of RAM, you have {}Mo installed", (usize::MAX / 1024) / 1024, (MEM_SIZE_BYTES / 1024) / 1024);
		}
		return MEM_SIZE_BYTES as usize;
	}
}

pub fn get_largest_mem_seg(mmap: *const MultibootTagMmap, mmap_size: usize) -> usize
{
	let mut largest_free_mem_seg: usize = 0;
	let mut largest_free_mem_seg_size: usize = 0;
	unsafe
	{
		for entry in (*mmap).entries(mmap_size)
		{
			if entry.len as usize > largest_free_mem_seg_size
			{
				largest_free_mem_seg_size = entry.len as usize;
				largest_free_mem_seg = entry.addr as usize;
			}
		}
	}
	largest_free_mem_seg
}
