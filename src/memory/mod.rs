pub mod pageframe;

use core::ffi::c_void;
use crate::multiboot::MultibootTagMmap;

extern "C"
{
	static _kernel_start: c_void;
	static _kernel_end: c_void;
}

pub fn get_mem_size(mmap: *const MultibootTagMmap, mmap_size: usize) -> usize
{
	let mut mem_size_bytes: u64 = 0;
	unsafe
	{
		crate::logln!("\x1B[33mmmap: {:#x?}\x1B[39m", (*mmap).entries(mmap_size));
		for mmap_entry in (*mmap).entries(mmap_size)
		{
			mem_size_bytes += mmap_entry.len as u64;
		}
		if mem_size_bytes > usize::MAX as u64
		{
			panic!("This version of ElsOS is in 32 bit, it only supports {}Mo of RAM, you have {}Mo installed", (usize::MAX / 1024) / 1024, (mem_size_bytes / 1024) / 1024);
		}
		return mem_size_bytes as usize;
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
