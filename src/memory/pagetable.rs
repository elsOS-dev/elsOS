use core::ffi::c_void;
use crate::libc;
use crate::memory::{page, pageframe, page_map_indexer};
use crate::memory::PAGE_SIZE;
use super::MemorySpace;

pub mod flags
{
	pub const PDE_PRESENT: usize = 0b0000_0001;
	pub const PDE_RW: usize = 0b0000_0010;
	pub const PDE_US: usize = 0b0000_0100;
	pub const PDE_PWT: usize = 0b0000_1000;
	pub const PDE_PCD: usize = 0b0001_0000;
	pub const PDE_ACCESSED: usize = 0b0010_0000;
	pub const PDE_FLAG2: usize = 0b0100_0000;
	pub const PDE_PS: usize = 0b1000_0000;

	pub const PTE_PRESENT: usize = 0b0000_0001;
	pub const PTE_RW: usize = 0b0000_0010;
	pub const PTE_US: usize = 0b0000_0100;
	pub const PTE_PWT: usize = 0b0000_1000;
	pub const PTE_PCD: usize = 0b0001_0000;
	pub const PTE_ACCESSED: usize = 0b0010_0000;
	pub const PTE_DIRTY: usize = 0b0100_0000;
	pub const PTE_PAT: usize = 0b1000_0000;
	pub const PTE_GLOBAL: usize = 0b1_0000_0000;
}

pub struct Manager
{
	pub page_directory: &'static mut [page::DirectoryEntry],
	paging_enabled: bool,
	flags: usize,
	pub memory_start: usize,
	pub page_count: usize,
	pub heap_start: usize,
	pub last_mapped: usize
}

impl Manager
{
	pub const fn uninitialized() -> Manager
	{
		Manager
		{
			page_directory: &mut [],
			paging_enabled: false,
			flags: 0,
			memory_start: 0,
			page_count: 0,
			heap_start: 0,
			last_mapped: 0
		}
	}

	pub fn new(addr: usize, flags: usize) -> Manager
	{
		unsafe
		{
			libc::memset(addr as *mut c_void, 0, PAGE_SIZE);
			let manager = Manager
			{
				page_directory: core::slice::from_raw_parts_mut(addr as *mut page::DirectoryEntry, 1024),
				paging_enabled: false,
				flags: flags,
				memory_start: 0,
				page_count: 0,
				heap_start: 0,
				last_mapped: 0
			};
			manager.page_directory[1023].set_addr(addr as u32);
			manager.page_directory[1023].set_rw(true);
			manager.page_directory[1023].set_present(true);
			manager
		}
	}

	pub unsafe fn enable_paging(&mut self)
	{
		self.remap_page_directory();
		self.paging_enabled = true;
	}

	unsafe fn remap_page_directory(&mut self)
	{
		self.page_directory = core::slice::from_raw_parts_mut(0xffff_f000 as *mut page::DirectoryEntry, 1024)
	}

	fn create_page_directory_entry(&mut self, index: usize)
	{
		let page_directory_entry = &mut self.page_directory[index];

		if !page_directory_entry.get_present()
		{
			let alloc = pageframe::Allocator::shared();
			page_directory_entry.reset();
			page_directory_entry.set_addr(alloc.request_free_page(MemorySpace::Kernel) as u32);
			page_directory_entry.value |= self.flags as u32 & 0xFFF;
			page_directory_entry.set_present(true);
			unsafe
			{
				libc::memset(self.address(index) as *mut c_void, 0, PAGE_SIZE);
			}
		}
	}

	fn create_page_table_entry(&mut self, page_directory_index: usize, page_table_index: usize, physical_address: usize)
	{
		let page_table = unsafe
		{
			core::slice::from_raw_parts_mut(self.address(page_directory_index) as *mut page::TableEntry, 1024)
		};
		let page_table_entry = &mut page_table[page_table_index];
		if !page_table_entry.get_present()
		{
			page_table_entry.reset();
			page_table_entry.set_addr(physical_address as u32);
			page_table_entry.value |= self.flags as u32 & 0xFFF;
			page_table_entry.set_present(true);
		}
	}

	pub fn memory_map(&mut self, v_addr: usize, phys_addr: usize)
	{
		let (pdi, pti): (usize, usize) = page_map_indexer(v_addr);
		self.create_page_directory_entry(pdi);
		self.create_page_table_entry(pdi, pti, phys_addr);
		if v_addr != phys_addr
		{
			if self.heap_start == 0
			{
				self.heap_start = v_addr;
				crate::serial_println!("[INFO] heap start at {:#08x}", self.heap_start);
			}
			self.last_mapped = v_addr;
		}
	}

	fn address(&self, page_directory_index: usize) -> u32
	{
		if self.paging_enabled
		{
			0xffc0_0000 + 0x1000 * page_directory_index as u32
		}
		else
		{
			let page_directory_entry = &self.page_directory[page_directory_index];
			page_directory_entry.get_addr()
		}
	}

	pub fn heap_size(&self) -> usize
	{
		self.last_mapped + 0xfff - self.heap_start
	}
}
