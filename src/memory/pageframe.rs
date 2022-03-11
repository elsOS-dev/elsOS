use crate::multiboot::MultibootTagMmap;
use crate::tools;
use crate::page_index;
use crate::memory::get_mem_size;

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
		self.reserved_mem = crate::memory::get_mem_size(mmap, mmap_size);
		crate::logln!("[INFO] found {}KiB of memory", self.reserved_mem / 1024);
		// initialise the bitmap according to mem size, and set every page as reserved
		self.init_bitmap(kernel_end);
		crate::logln!("[INFO] assigned {} pages to bitmap", self.bitmap.size);
		unsafe
		{
			for entry in (*mmap).entries(mmap_size)
			{
				if (entry.tag_type == 1 && entry.addr < get_mem_size(mmap, mmap_size) as u32) && entry.addr != 0
				{
					// unreserve grub memmap entries marked as free except lower memory
					crate::logln!("unreserving {} pages", page_index!(entry.len as usize));
					self.unreserve_mem(page_index!(entry.addr as usize), page_index!(entry.len as usize));
				}
			}
		}
		// reserve kernel space
		crate::logln!("reserving {} pages for kernel", page_index!(kernel_end - kernel_start));
		self.reserve_mem(page_index!(kernel_start), page_index!(kernel_end - kernel_start));
		crate::logln!("kernel end {:#x}", kernel_end);
		crate::logln!("memory start {:#x}", kernel_end + self.bitmap.size / 8);
		// reserve bitmap
		crate::logln!("reserving {} pages for bitmap", page_index!(self.bitmap.size / 8));
		self.reserve_mem(page_index!(kernel_end),  page_index!(self.bitmap.size / 8));
	}

	pub fn print_memusage(&self, level: usize)
	{

		crate::logln!("[INFO] free mem: {}KiB", self.free_mem / 1024);
		crate::logln!("[INFO] used mem: {}KiB", self.locked_mem / 1024);
		if level >= 1
		{
			crate::logln!("[INFO] reserved mem: {}KiB", self.reserved_mem / 1024);
		}
		if level >= 2
		{
			crate::logln!("[INFO] reserved pages: {} pages", self.reserved_mem / 4096);
			crate::logln!("[INFO] used pages: {} pages", self.locked_mem / 4096);
		}
		if level >= 3
		{
			crate::logln!("excepted levels for print_memusage() are 0, 1 or 2.");
		}
	}

	fn reserve_mem(&mut self, index: usize, len: usize)
	{
		for i in 0..len
		{
			if self.bitmap[index + i] == false
			{
				self.bitmap.set(index + i, true);
				self.reserved_mem += 4096;
				self.free_mem -= 4096;
			}
		}
	}

	fn unreserve_mem(&mut self, index: usize, len: usize)
	{
		for i in 0..len
		{
			self.bitmap.set(index + i, false);
			self.reserved_mem -= 4096;
			self.free_mem += 4096;
		}
	}
	fn init_bitmap(&mut self, b: usize)
	{
		let bitmap_size = self.reserved_mem / 4096;
		crate::logln!("[INFO] bitmap location: {:#x}", b);

		unsafe
		{
			self.bitmap = tools::Bitmap
			{
				buffer: core::slice::from_raw_parts_mut (b as *mut u8, (bitmap_size / 8) + 8),
				size: bitmap_size,
			};

			self.bitmap.erase();
			// self.bitmap.debug_print(256);
		}
	}
}
