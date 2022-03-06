use crate::multiboot::MultibootTagMmap;
use crate::tools;

use core::ffi::c_void;

extern "C"
{
	static _kernel_start: c_void;
	static _kernel_end: c_void;
}

pub struct PageFrameAllocator
{
	pub free_mem: u64,
	pub locked_mem: u32,
	pub reserved_mem: u32,
	pub unusable_mem: u32,
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
		let mut largest_free_mem_seg: u64 = 0;
		let mut largest_free_mem_seg_size: u64 = 0;

		if self.initialized
		{
			return ;
		}
		self.initialized = true;
		unsafe
		{
			for entry in (*mmap).entries(mmap_size)
			{
				if entry.len > largest_free_mem_seg_size
				{
					largest_free_mem_seg_size = entry.len;
					largest_free_mem_seg = entry.addr;
				}
			}
			self.free_mem = crate::memory::get_mem_size(mmap, mmap_size);
			self.init_bitmap(&_kernel_end as *const _ as usize + largest_free_mem_seg as usize + 1);
		}
	}

	fn init_bitmap(&mut self, b: usize)
	{
		let bitmap_size = self.free_mem / 4096;

		unsafe
		{
			self.bitmap = tools::Bitmap
			{
				buffer: core::slice::from_raw_parts_mut (b as *mut u8, (bitmap_size / 8) as usize + 1),
				size: bitmap_size as usize + 1,
			};
			self.bitmap.erase();
			crate::logln!("{}", self.bitmap.size);
			self.bitmap.debug_print();
		}
	}
}
