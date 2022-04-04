use crate::libc;
use crate::memory::PAGE_SIZE;
use crate::memory::page;
use crate::memory::pageframe;
use crate::memory::page_map_indexer;
use core::ffi::c_void;

pub struct Manager
{
	pub page_directory: &'static mut [page::DirectoryEntry],
}

impl Manager
{
	pub fn new(addr: usize) -> Manager
	{
		unsafe
		{
			libc::memset(addr as *mut c_void, 0, PAGE_SIZE);
			Manager
			{
				page_directory: core::slice::from_raw_parts_mut(addr as *mut page::DirectoryEntry, 1024)
			}
		}
	}

	fn create_page_directory_entry(&mut self, index: usize)
	{
		let page_directory_entry = &mut self.page_directory[index];

		if !page_directory_entry.get_present()
		{
			let alloc = pageframe::Allocator::shared();
			let page_table_addr = alloc.request_free_page();
			unsafe
			{
				libc::memset(page_table_addr as *mut c_void, 0, PAGE_SIZE);
			}
			page_directory_entry.set_addr(page_table_addr as u32);
			page_directory_entry.set_rw(true);
			page_directory_entry.set_present(true);
		}
	}

	fn create_page_table_entry(&mut self, page_directory_index: usize, page_table_index: usize, physical_address: usize)
	{
		let page_directory_entry = &mut self.page_directory[page_directory_index];
		let page_table = unsafe
		{
			core::slice::from_raw_parts_mut(page_directory_entry.get_addr() as *mut page::TableEntry, 1024)
		};
		let page_table_entry = &mut page_table[page_table_index];

		if !page_table_entry.get_present()
		{
			page_table_entry.set_addr(physical_address as u32);
			page_table_entry.set_rw(true);
			page_table_entry.set_present(true);
		}
	}

	pub fn memory_map(&mut self, v_addr: usize, phys_addr: usize)
	{
		let (pdi, pti): (usize, usize) = page_map_indexer(v_addr);

		self.create_page_directory_entry(pdi);
		self.create_page_table_entry(pdi, pti, phys_addr);
	}
}
