mod pageframe;
mod pagetable;
mod page;

use crate::multiboot::MultibootTagMmap;

static PAGE_SIZE: usize = 4096;

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
			panic!("This version of elsOS is in 32 bit, it only supports {}MiB of RAM, you have {}MiB installed", (usize::MAX / 1024) / 1024, (MEM_SIZE_BYTES / 1024) / 1024);
		}
		return MEM_SIZE_BYTES as usize;
	}
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
	fn load_page_directory(address: *const page::DirectoryEntry);
	fn enable_paging();
}

fn create_pd(addr: usize, pt1_addr: usize)
{
	let pd: &'static mut [page::DirectoryEntry];
	let pt1: &'static mut [page::TableEntry];

	pd = unsafe{ core::slice::from_raw_parts_mut(addr as *mut page::DirectoryEntry, 1024) };
	pt1 = unsafe{ core::slice::from_raw_parts_mut(pt1_addr as *mut page::TableEntry, 1024) };
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
		crate::logln!("{:?}", addr as *const page::DirectoryEntry);
		load_page_directory(addr as *const page::DirectoryEntry);
		enable_paging();
	}
}

// mapping the first page table to physical memory.
fn id_paging(start: &mut [page::TableEntry])
{
	let mut block: usize = 0;
	for table in start
	{
		table.set_addr((block) as u32);
		block += 0x1000;
	}
}

pub fn page_map_indexer(v_addr: usize) -> (usize, usize)
{
	let pdindex = v_addr >> 22;
	let ptindex = v_addr >> 12 & 0x03FF;

	return (pdindex, ptindex);
}