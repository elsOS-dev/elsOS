use crate::multiboot::MultibootTagMmap;
use crate::tools;
use crate::page_index;

use core::ffi::c_void;

extern "C"
{
	static _kernel_start: c_void;
	static _kernel_end: c_void;
}

pub struct PageFrameAllocator
{
	pub free_mem: usize,
	pub locked_mem: usize,
	pub reserved_mem: usize,
	pub unusable_mem: u64,
	initialized: bool,
	bitmap: tools::Bitmap,
}

impl PageFrameAllocator
{
	pub fn new() -> PageFrameAllocator
	{
		PageFrameAllocator
		{
			free_mem: 0,
			locked_mem: 0,
			reserved_mem: 0,
			unusable_mem: 0,
			initialized: false,
			bitmap: tools::Bitmap {buffer: &mut[] as &'static mut[u8], size: 0},

		}
	}

	pub fn read_grub_mmap(&mut self, mmap: *const MultibootTagMmap, mmap_size: usize)
	{
		let kernel_start: usize;
		let kernel_end: usize;

		if self.initialized
		{
			return ;
		}
		self.initialized = true;

		crate::logln!("[INFO] initializing memory map...");
		unsafe
		{
			kernel_start = &_kernel_start as *const _ as usize;
			kernel_end =  &_kernel_end as *const _ as usize;
		}
		self.free_mem = crate::memory::get_mem_size(mmap, mmap_size);
		crate::logln!("[INFO] found {}Ko of memory", self.free_mem / 1024);
		self.init_bitmap(kernel_end + 1);
		crate::logln!("[INFO] assigned {} pages to bitmap", self.bitmap.size);
		unsafe
		{
			for entry in (*mmap).entries(mmap_size)
			{
				if entry.tag_type != 1
				{
					self.reserve_mem(page_index!(entry.addr as usize), page_index!(entry.len as usize));
				}
			}
		}
		self.reserve_mem(page_index!(kernel_start), page_index!(kernel_end - kernel_start));
		self.reserve_mem(page_index!(kernel_end),  page_index!(self.bitmap.size) / 8);
		crate::logln!("[INFO] reserved pages: {} pages", self.reserved_mem / 4096);
		crate::logln!("[INFO] reserved mem: {}Ko", self.reserved_mem / 1024);
	}

	fn reserve_mem(&mut self, index: usize, len: usize)
	{
		for i in 0..len
		{
			self.bitmap.set((index + i) as usize, true);
			self.reserved_mem += 4096;
		}
	}
	fn init_bitmap(&mut self, b: usize)
	{
		let bitmap_size = usize::MAX / 4096;

		unsafe
		{
			self.bitmap = tools::Bitmap
			{
				buffer: core::slice::from_raw_parts_mut (b as *mut u8, (bitmap_size / 8) + 8),
				size: bitmap_size + 1,
			};

			self.bitmap.erase();
			// self.bitmap.debug_print();
		}
	}
}
