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

	let page_directory_addr = alloc.request_free_page(true);
	let mut pt_manager = pagetable::Manager::new(page_directory_addr);

	id_map(&mut pt_manager);
	alloc.print_memusage(1);
	unsafe
	{
		load_page_directory(page_directory_addr as *const page::DirectoryEntry);
		enable_paging();
		pt_manager.enable_paging();
	}
	let page = alloc.request_free_page(false);
	crate::logln!("requested page userspace: {:#0X}", page);
	pt_manager.memory_map(0x150000, page, 3);
}

fn id_map(pt_manager: &mut pagetable::Manager)
{
	let alloc: &mut pageframe::Allocator = pageframe::Allocator::shared();
	let mut memory_start = alloc.bitmap.buffer as *const _ as *const usize as usize;
	memory_start += alloc.bitmap.buffer.len();

	for i in 0..memory_start / PAGE_SIZE
	{
		pt_manager.memory_map(i * PAGE_SIZE, i * PAGE_SIZE, 3);
		alloc.lock_page(i);
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
