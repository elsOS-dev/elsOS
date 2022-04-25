use core::cmp::max;
use core::ffi::c_void;
use super::pageframe;
use super::pagetable;
use super::PAGE_SIZE;
use super::PT_MANAGER;
use crate::ferramenta;
use crate::libc;

#[repr(align(0x10))]
struct AllocHeader
{
	magic: u16,
	size: usize,
	freed: bool
}

pub fn kmalloc(size: usize) -> *mut c_void
{
	let size = ferramenta::align(size, 0x10);
	let pt_manager: &mut pagetable::Manager = unsafe
	{
		&mut PT_MANAGER
	};

	let address = next_available_space(pt_manager.memory_start, size, true);

	if address == 0
	{
		crate::oops!("kernel space out of memory");
		return core::ptr::null_mut::<c_void>();
	}
	let header: *mut AllocHeader = address as *mut _;

	unsafe
	{
		(*header).magic = 0x4242;
		(*header).size = size;
		(*header).freed = false;
	}
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
	free(address, true);
}

pub fn ksize(address: *mut c_void) -> usize
{
	size_of(address, true)
}

pub fn vmalloc(size: usize) -> *mut c_void
{
	let size = ferramenta::align(size, 0x10);
	let pt_manager: &mut pagetable::Manager = unsafe
	{
		&mut PT_MANAGER
	};

	let mut address = next_available_space(pt_manager.heap_start, size, false);

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
	let header: *mut AllocHeader = address as *mut _;

	unsafe
	{
		(*header).magic = 0x4242;
		(*header).size = size;
		(*header).freed = false;
	}
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
	free(address, false);
}

pub fn vsize(address: *mut c_void) -> usize
{
	size_of(address, false)
}

// Private functions

fn next_available_space(address: usize, size: usize, kernel_space: bool) -> usize
{
	let pt_manager: &mut pagetable::Manager = unsafe
	{
		&mut PT_MANAGER
	};
	let total_size = size + core::mem::size_of::<AllocHeader>();
	let mut max_i = if kernel_space
	{
		super::KERNEL_SPACE_START * PAGE_SIZE + super::KERNEL_SPACE_RANGE * PAGE_SIZE - pt_manager.memory_start
	}
	else
	{
		pt_manager.last_mapped + 0x1000 - pt_manager.heap_start
	};
	let mut i = 0x0;

	loop
	{
		while i + total_size > max_i
		{
			let pages = max(total_size / PAGE_SIZE, 1);
			crate::logln!("trying to get {} pages", pages);
			if let Some(_) = expand_heap(pages)
			{
				max_i = pt_manager.last_mapped + 0x1000 - pt_manager.heap_start;
			}
			else
			{
				return 0;
			}
		}
		unsafe
		{
			let header: *mut AllocHeader = (address + i) as *mut _;

			if (*header).magic == 0x4242
			{
				if (*header).freed
				{
					merge_next_blocks(get_block_for(&*header), kernel_space);
					if (*header).size > size
					{
						let new_address = address + i + core::mem::size_of::<AllocHeader>();
						let could_break = break_block(new_address as *mut c_void, size, kernel_space);

						if could_break
						{
							return header as usize;
						}
					}
					else if (*header).size == size
					{
						return header as usize;
					}
				}
			}
			else
			{
				return address + i;
			}
			i += (*header).size + core::mem::size_of::<AllocHeader>();
		}
	}
}

fn free(address: *mut c_void, kernel_space: bool)
{
	if address != core::ptr::null_mut::<c_void>()
	{
		if let Some(header) = get_header_for(address, kernel_space)
		{
			if header.freed
			{
				crate::oops!("double free");
			}
			else
			{
				header.freed = true;
				merge_next_blocks(address, kernel_space);
			}
		}
	}
}

fn size_of(address: *mut c_void, kernel_space: bool) -> usize
{
	if let Some(header) = get_header_for(address, kernel_space)
	{
		header.size
	}
	else
	{
		0
	}
}

fn get_header_for(address: *mut c_void, kernel_space: bool) -> Option<&'static mut AllocHeader>
{
	let pt_manager: &mut pagetable::Manager = unsafe
	{
		&mut PT_MANAGER
	};
	let lowest_address = if kernel_space
	{
		pt_manager.memory_start + core::mem::size_of::<AllocHeader>()
	}
	else
	{
		pt_manager.heap_start + core::mem::size_of::<AllocHeader>()
	};
	let highest_address = if kernel_space
	{
		PAGE_SIZE * (super::KERNEL_SPACE_START + super::KERNEL_SPACE_RANGE)
	}
	else
	{
		pt_manager.last_mapped + PAGE_SIZE
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
		Some(&mut *((address as usize - core::mem::size_of::<AllocHeader>()) as *mut _))
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
	let new_page = alloc.request_free_pages(max(pages, 1), false);

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

fn break_block(address: *mut c_void, new_size: usize, kernel_space: bool) -> bool
{
	if let Some(header) = get_header_for(address, kernel_space)
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

fn merge_next_blocks(address: *mut c_void, kernel_space: bool)
{
	if let Some(header) = get_header_for(address, kernel_space)
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
