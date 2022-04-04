use crate::memory::page;
use crate::memory::page_map_indexer;

struct Manager
{
	pub pd_addr: &'static mut [page::DirectoryEntry],
}

impl Manager
{
	pub fn new(&mut self, addr: usize)
	{
		self.pd_addr = unsafe{ core::slice::from_raw_parts_mut(addr as *mut page::DirectoryEntry, 1024) };
	}

	pub fn memory_map(&self, v_addr: usize, phys_addr: usize)
	{
		let pdi : usize;
		let pti : usize;

		(pdi, pti) = page_map_indexer(v_addr);
		let pd = &self.pd_addr[pdi];
		if !pd.get_present()
		{
			// create entry
			// request page
			// memset to 0
			// set flags
		}

		let pt = unsafe{ core::slice::from_raw_parts_mut(pd.get_addr() as *mut page::TableEntry, 1024) };
		let pte = &mut pt[pti];
		if !pte.get_present()
		{
			// create entry
			// request page
			// memset to 0
			// set flags
		}
		pte.set_addr(phys_addr as u32);
	}
}