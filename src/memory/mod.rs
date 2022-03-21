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

pub fn init(mmap: *const MultibootTagMmap, mmap_size: usize)
{
	let pd_ptr: usize;
	let pt1_ptr: usize;
	let mut alloc: pageframe::Allocator = pageframe::Allocator::new();

	alloc.read_grub_mmap(mmap, mmap_size);
	pd_ptr = alloc.request_free_page();
	pt1_ptr = alloc.request_free_page();
	create_pd(pd_ptr, pt1_ptr);
	crate::logln!("test");
}


extern "C"
{
	fn load_page_directory(address: *const u8);
	fn enable_paging();
}

fn create_pd(addr: usize, pt1_addr: usize)
{
	let pd: &'static mut [pagedirectory::PageDirectoryEntry];
	let pt1: &'static mut [pagetable::PageTableEntry];

	pd = unsafe{ core::slice::from_raw_parts_mut(addr as *mut pagedirectory::PageDirectoryEntry, 1024) };
	pt1 = unsafe{ core::slice::from_raw_parts_mut(pt1_addr as *mut pagetable::PageTableEntry, 1024) };
	for d in &mut *pd
	{
		d.value = 0x00000002;
	}
	for (i, t) in &mut (*pt1).iter_mut().enumerate()
	{
		t.value = ((i * 0x1000) | 3) as u32;
	}
	pd[0].set_addr(pt1_addr as u32);
	id_paging(pt1);
	pd[0].set_present(true);
	crate::logln!("\x1b[31m{:#x?}\x1b[39m", pt1[0]);
	unsafe
	{
		crate::logln!("{:?}", addr as *const u8);
		load_page_directory(addr as *const u8);
		enable_paging();
	}
}

// mapping the first page table to physical memory.
fn id_paging(start: &mut [pagetable::PageTableEntry])
{
	let mut block: usize = 0;
	for table in start
	{
		table.set_addr((block) as u32);
		block += 0x1000;
	}
}
