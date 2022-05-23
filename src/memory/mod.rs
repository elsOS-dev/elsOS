use crate::ferramenta;
use crate::libc;
use crate::multiboot::MultibootTagMmap;
use pagetable::flags::*;
pub use malloc::*;

pub mod malloc;
mod page;
mod pageframe;
mod pagetable;

// In pages, * PAGE_SIZE to get memory sizes
const KERNEL_SPACE_START: usize = 0x0000_0000;
const KERNEL_SPACE_RANGE: usize = 0x0000_2000;

static PAGE_SIZE: usize = 4096;
static mut PT_MANAGER: pagetable::Manager = pagetable::Manager::uninitialized();

#[derive(Copy, Clone, PartialEq)]
pub enum MemorySpace
{
	Kernel,
	User
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
		for mmap_entry in (*mmap).entries(mmap_size)
		{
			MEM_SIZE_BYTES += mmap_entry.len as u64;
		}
		let installed = (MEM_SIZE_BYTES / 1024) / 1024;
		let mini = KERNEL_SPACE_RANGE * PAGE_SIZE + 0x100000;
		if MEM_SIZE_BYTES > usize::MAX as u64
		{
			panic!("This version of elsOS is in 32 bit, it only supports {}MiB of RAM,\nyou have {}MiB installed", (usize::MAX / 1024) / 1024, installed);
		}
		else if MEM_SIZE_BYTES < mini as u64
		{
			panic!("This version of elsOS needs at least {}MiB of RAM,\nyou have {}MiB installed", mini / 1024 / 1024, installed);
		}
		return MEM_SIZE_BYTES as usize;
	}
}

pub fn init(mmap: *const MultibootTagMmap, mmap_size: usize)
{
	let alloc: &mut pageframe::Allocator = pageframe::Allocator::shared();
	alloc.read_grub_mmap(mmap, mmap_size);

	let page_directory_addr = alloc.request_free_page(MemorySpace::Kernel);
	let mut pt_manager = pagetable::Manager::new(page_directory_addr, PDE_RW);

	pt_manager.page_count = alloc.bitmap.size;
	id_map(&mut pt_manager);
	alloc.print_memusage(1);
	unsafe
	{
		load_page_directory(page_directory_addr as *const page::DirectoryEntry);
		enable_paging();
		pt_manager.enable_paging();
	}
	let page = alloc.request_free_page(MemorySpace::User);
	let virtual_page = PAGE_SIZE * (KERNEL_SPACE_START + KERNEL_SPACE_RANGE);
	pt_manager.memory_map(virtual_page, page);
	unsafe
	{
		libc::memset(virtual_page as *mut _, 0, PAGE_SIZE);
	}

	unsafe
	{
		PT_MANAGER = pt_manager;
	}
}

fn id_map(pt_manager: &mut pagetable::Manager)
{
	let alloc: &mut pageframe::Allocator = pageframe::Allocator::shared();
	let mut memory_start = alloc.bitmap.buffer as *const _ as *const usize as usize;
	memory_start += alloc.bitmap.buffer.len();

	pt_manager.memory_start = ferramenta::align(memory_start, PAGE_SIZE);
	for i in 0..memory_start / PAGE_SIZE
	{
		pt_manager.memory_map(i * PAGE_SIZE, i * PAGE_SIZE);
		alloc.lock_page(i);
	}
	for i in memory_start / PAGE_SIZE..KERNEL_SPACE_START + KERNEL_SPACE_RANGE
	{
		pt_manager.memory_map(i * PAGE_SIZE, i * PAGE_SIZE);
	}
	for i in pt_manager.page_count / 1024 + 2 + memory_start / PAGE_SIZE..KERNEL_SPACE_START + KERNEL_SPACE_RANGE
	{
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

pub fn is_range_mapped(ptr: *const u8, n: usize) -> bool
{
	let pt_manager = unsafe
	{
		&PT_MANAGER
	};
	if ptr as usize >= pt_manager.last_mapped + PAGE_SIZE || ptr as usize + n > pt_manager.last_mapped + PAGE_SIZE
	{
		false
	}
	else
	{
		true
	}
}
