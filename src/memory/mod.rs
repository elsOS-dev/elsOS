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
	let alloc: &mut pageframe::Allocator = pageframe::Allocator::shared();
	alloc.read_grub_mmap(mmap, mmap_size);

	let page_directory_addr = alloc.request_free_page();
	let mut pt_manager = pagetable::Manager::new(page_directory_addr);

	id_map(&mut pt_manager);
	unsafe
	{
		load_page_directory(page_directory_addr as *const page::DirectoryEntry);
		enable_paging();
	}
}

fn id_map(pt_manager: &mut pagetable::Manager)
{
	for i in 0..1024
	{
		pt_manager.memory_map(i * PAGE_SIZE, i * PAGE_SIZE);
		pageframe::Allocator::shared().lock_page(i);
	}
}

extern "C"
{
	fn load_page_directory(address: *const page::DirectoryEntry);
	fn enable_paging();
}

pub fn page_map_indexer(v_addr: usize) -> (usize, usize)
{
	let pdindex = v_addr >> 22;
	let ptindex = v_addr >> 12 & 0x03FF;

	return (pdindex, ptindex);
}
