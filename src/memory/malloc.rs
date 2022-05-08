use core::cmp::max;
use core::ffi::c_void;
use crate::ferramenta;
use crate::libc;
use super::{pageframe, pagetable};
use super::MemorySpace;
use super::{PAGE_SIZE, PT_MANAGER};

#[repr(align(0x10))]
struct AllocHeader
{
	magic: u16,
	size: usize,
	freed: bool
}

pub fn kmalloc(size: usize) -> *mut c_void
{
	if size == 0
	{
		crate::oops!("cannot allocate memory of size 0");
		return core::ptr::null_mut::<c_void>();
	}

	let size = ferramenta::align(size, 0x10);
	let pt_manager: &mut pagetable::Manager = unsafe
	{
		&mut PT_MANAGER
	};

	let start = pt_manager.memory_start + PAGE_SIZE * (pt_manager.page_count / 1024 + 2);
	let address = next_available_space(start, size, MemorySpace::Kernel);

	if address == 0
	{
		crate::oops!("kernel space out of memory");
		return core::ptr::null_mut::<c_void>();
	}
	create_header(address, size);
	(address + core::mem::size_of::<AllocHeader>()) as *mut c_void
}

pub fn kzalloc(size: usize) -> *mut c_void
{
	let size = ferramenta::align(size, 0x10);
	let address = kmalloc(size);

	if address != core::ptr::null_mut::<c_void>()
	{
		unsafe
		{
			libc::memset(address, 0, size);
		}
	}
	address
}

pub fn kfree(address: *mut c_void)
{
	free(address, MemorySpace::Kernel);
}

pub fn ksize(address: *mut c_void) -> usize
{
	size_of(address, MemorySpace::Kernel)
}

pub fn vmalloc(size: usize) -> *mut c_void
{
	if size == 0
	{
		crate::oops!("cannot allocate memory of size 0");
		return core::ptr::null_mut::<c_void>();
	}

	let size = ferramenta::align(size, 0x10);
	let pt_manager: &mut pagetable::Manager = unsafe
	{
		&mut PT_MANAGER
	};

	let mut address = next_available_space(pt_manager.heap_start, size, MemorySpace::User);

	if address == 0
	{
		if let Some(new_address) = expand_heap(200)
		{
			address = new_address as usize;
		}
		else
		{
			crate::oops!("virtual memory out of memory");
			return core::ptr::null_mut::<c_void>();
		}
	}
	create_header(address, size);
	(address + core::mem::size_of::<AllocHeader>()) as *mut c_void
}

pub fn vzalloc(size: usize) -> *mut c_void
{
	let size = ferramenta::align(size, 0x10);
	let address = vmalloc(size);

	if address != core::ptr::null_mut::<c_void>()
	{
		unsafe
		{
			libc::memset(address, 0, size);
		}
	}
	address
}

pub fn vfree(address: *mut c_void)
{
	free(address, MemorySpace::User);
}

pub fn vsize(address: *mut c_void) -> usize
{
	size_of(address, MemorySpace::User)
}

// Private functions

fn create_header(address: usize, size: usize)
{
	let header: *mut AllocHeader = address as *mut _;

	unsafe
	{
		(*header).magic = 0x4242;
		(*header).size = size;
		(*header).freed = false;
	}
}

fn next_available_space(address: usize, size: usize, memory_space: MemorySpace) -> usize
{
	let pt_manager: &mut pagetable::Manager = unsafe
	{
		&mut PT_MANAGER
	};
	let total_size = size + core::mem::size_of::<AllocHeader>();
	let limit = match memory_space
	{
		MemorySpace::Kernel => super::KERNEL_SPACE_START * PAGE_SIZE + super::KERNEL_SPACE_RANGE * PAGE_SIZE - pt_manager.memory_start,
		MemorySpace::User => pt_manager.heap_size()
	};
	let mut start = 0x0;

	loop
	{
		if memory_space == MemorySpace::Kernel && (start + total_size > limit || address + start + total_size > super::KERNEL_SPACE_START * PAGE_SIZE + super::KERNEL_SPACE_RANGE * PAGE_SIZE)
		{
			return 0;
		}
		else if !expand_heap_if_needed(start, total_size, limit)
		{
			return 0;
		}
		unsafe
		{
			let header: *mut AllocHeader = (address + start) as *mut _;
			if (*header).magic == 0x4242
			{
				if (*header).freed
				{
					if use_freed_block(address + start, size, memory_space)
					{
						return header as usize;
					}
				}
			}
			else
			{
				return address + start;
			}
			start += (*header).size + core::mem::size_of::<AllocHeader>();
		}
	}
}

fn expand_heap_if_needed(start: usize, size: usize, heap_size: usize) -> bool
{
	let mut heap_size = heap_size;
	let pt_manager: &mut pagetable::Manager = unsafe
	{
		&mut PT_MANAGER
	};

	while start + size > heap_size
	{
		let pages = max(size / PAGE_SIZE, 1);
		if let Some(_) = expand_heap(pages)
		{
			heap_size = pt_manager.heap_size();
		}
		else
		{
			return false;
		}
	}
	true
}

fn use_freed_block(address: usize, size: usize, memory_space: MemorySpace) -> bool
{
	unsafe
	{
		let header: *mut AllocHeader = address as *mut _;
		merge_next_blocks(get_block_for(&*header), memory_space);

		if (*header).size > size
		{
			let new_address = address + core::mem::size_of::<AllocHeader>();
			let could_break = break_block(new_address as *mut c_void, size, memory_space);

			if could_break
			{
				return true;
			}
		}
		else if (*header).size == size
		{
			return true;
		}
		false
	}
}

fn free(address: *mut c_void, memory_space: MemorySpace)
{
	if address != core::ptr::null_mut::<c_void>()
	{
		if let Some(header) = get_header_for(address, memory_space)
		{
			if header.freed
			{
				crate::oops!("double free");
			}
			else
			{
				header.freed = true;
				merge_next_blocks(address, memory_space);
			}
		}
	}
}

fn size_of(address: *mut c_void, memory_space: MemorySpace) -> usize
{
	if let Some(header) = get_header_for(address, memory_space)
	{
		if header.freed
		{
			crate::oops!("cannot get size of freed variable");
			0
		}
		else
		{
			header.size
		}
	}
	else
	{
		0
	}
}

fn get_header_for(address: *mut c_void, memory_space: MemorySpace) -> Option<&'static mut AllocHeader>
{
	let pt_manager: &mut pagetable::Manager = unsafe
	{
		&mut PT_MANAGER
	};
	let lowest_address = match memory_space
	{
		MemorySpace::Kernel => pt_manager.memory_start + core::mem::size_of::<AllocHeader>(),
		MemorySpace::User => pt_manager.heap_start + core::mem::size_of::<AllocHeader>()
	};
	let highest_address = match memory_space
	{
		MemorySpace::Kernel => PAGE_SIZE * (super::KERNEL_SPACE_START + super::KERNEL_SPACE_RANGE),
		MemorySpace::User => pt_manager.last_mapped + PAGE_SIZE
	};
	if (address as usize) < lowest_address
	{
		crate::oops!("cannot get memory header under heap_start ({:#08x}) (address: {:#08x})", lowest_address, address as usize);
		return None;
	}
	if (address as usize) > highest_address
	{
		crate::oops!("cannot get memory over memory end ({:#08x}) (address: {:#08x})", highest_address, address as usize);
		return None;
	}
	unsafe
	{
		let header: &'static mut AllocHeader = &mut *((address as usize - core::mem::size_of::<AllocHeader>()) as *mut _);
		if header.magic == 0x4242
		{
			Some(header)
		}
		else
		{
			None
		}
	}
}

fn get_block_for(header_address: &AllocHeader) -> *mut c_void
{
	(header_address as *const _ as usize + core::mem::size_of::<AllocHeader>()) as *mut c_void
}

fn expand_heap(pages: usize) -> Option<*mut c_void>
{
	let pt_manager = unsafe
	{
		&mut PT_MANAGER
	};
	let alloc = pageframe::Allocator::shared();
	let new_page = alloc.request_free_pages(max(pages, 1), MemorySpace::User);

	if new_page != 0
	{
		for i in 0..pages
		{
			pt_manager.memory_map(pt_manager.last_mapped + PAGE_SIZE, new_page + i * PAGE_SIZE);
			unsafe
			{
				libc::memset((pt_manager.last_mapped) as *mut c_void, 0, PAGE_SIZE);
			}
		}
		return Some(new_page as *mut c_void);
	}
	None
}

fn break_block(address: *mut c_void, new_size: usize, memory_space: MemorySpace) -> bool
{
	if let Some(header) = get_header_for(address, memory_space)
	{
		let old_size = header.size;

		if old_size - new_size > core::mem::size_of::<AllocHeader>()
		{
			let new_header: &'static mut AllocHeader = unsafe
			{
				&mut *((address as usize + new_size) as *mut _)
			};

			header.size = new_size;
			new_header.magic = 0x4242;
			new_header.size = old_size - new_size - core::mem::size_of::<AllocHeader>();
			new_header.freed = true;

			return true;
		}
	}
	false
}

fn merge_next_blocks(address: *mut c_void, memory_space: MemorySpace)
{
	if let Some(header) = get_header_for(address, memory_space)
	{
		loop
		{
			let size = header.size;
			let next_header: &'static mut AllocHeader = unsafe
			{
				&mut *((address as usize + size) as *mut _)
			};

			if next_header.magic == 0x4242 && next_header.freed
			{
				header.size += core::mem::size_of::<AllocHeader>() + next_header.size;
				next_header.magic = 0;
				next_header.size = 0;
				next_header.freed = false;
			}
			else
			{
				break;
			}
		}
	}
}
