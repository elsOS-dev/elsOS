use crate::tools;

pub struct PageDirectoryEntry
{
	pub value: u32
}

impl core::fmt::Debug for PageDirectoryEntry
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		f.debug_struct("PageDirectoryEntry")
		.field("value", &self.value)
		.field("addr", &self.get_addr())
		.field("flags", &self.get_flags())
		.field("ps", &self.get_ps())
		.field("flag2", &self.get_flag2())
		.field("accessed", &self.get_accessed())
		.field("pcd", &self.get_pcd())
		.field("pwt", &self.get_pwt())
		.field("us", &self.get_us())
		.field("rw", &self.get_rw())
		.field("present", &self.get_present())
		.finish()
	}
}

impl PageDirectoryEntry
{
	// Bits 31-12 represents the address to the PageTableDirectory.
	pub fn get_addr(&self) -> u32
	{
		self.value >> 12
	}
	pub fn set_addr(&mut self, addr: u32)
	{
		self.value &= 0xfff;
		self.value |= addr;
	}

	// Bits 11-8 are available for us to do whatever we want.
	pub fn get_flags(&self) -> u8
	{
		((self.value << 20) >> 28) as u8
	}
	pub fn set_flags(&mut self, flags: u8)
	{
		self.value &= 0xffff_f0ff;
		self.value |= (flags as u32) << 8;
	}

	// Bit 7
	// PS, or 'Page Size' stores t-he page size for that specific entry.
	// If the bit is set, then the PDE maps to a page that is 4 MiB in size.
	// Otherwise, it maps to a 4 KiB page table. Please note that 4-MiB pages
	// require PSE to be enabled. In our case, it will be 0.
	pub fn get_ps(&self) -> bool
	{
		tools::get_bit_at(self.value, 7)
	}
	pub fn set_ps(&mut self, value: bool)
	{
		tools::set_bit(&mut self.value, value, 7);
	}


	// Bit 6 is available for us to do whatever we want.
	pub fn get_flag2(&self) -> bool
	{
		tools::get_bit_at(self.value, 6)
	}
	pub fn set_flag2(&mut self, value: bool)
	{
		tools::set_bit(&mut self.value, value, 6);
	}

	// Bit 5
	// 'Accessed' is used to discover whether a PDE or PTE was read during
	// virtual address translation. If it has, then the bit is set, otherwise,
	// it is not. Note that, this bit will not be cleared by the CPU,
	// so that burden falls on the OS (if it needs this bit at all).
	pub fn get_accessed(&self) -> bool
	{
		tools::get_bit_at(self.value, 5)
	}
	pub fn set_accessed(&mut self, value: bool)
	{
		tools::set_bit(&mut self.value, value, 5);
	}

	// Bit 4
	// PCD, is the 'Cache Disable' bit. If the bit is set, the page will not be
	// cached. Otherwise, it will be.
	pub fn get_pcd(&self) -> bool
	{
		tools::get_bit_at(self.value, 4)
	}
	pub fn set_pcd(&mut self, value: bool)
	{
		tools::set_bit(&mut self.value, value, 4);
	}

	// Bit 3
	// PWT, controls Write-Through' abilities of the page. If the bit is set,
	// write-through caching is enabled. If not, then write-back is enabled
	// instead.
	pub fn get_pwt(&self) -> bool
	{
		tools::get_bit_at(self.value, 3)
	}
	pub fn set_pwt(&mut self, value: bool)
	{
		tools::set_bit(&mut self.value, value, 3);
	}

	// Bit 2
	// The 'User/Supervisor' bit, controls access to the page based on privilege
	// level. If the bit is set, then the page may be accessed by all;
	// if the bit is not set, however, only the supervisor can access it. 
	// For a page directory entry, the user bit controls access to all the pages
	// referenced by the page directory entry. T
	// herefore if you wish to make a page a user page, you must set the user 
	// bit in the relevant page directory entry as well as the page table entry
	pub fn get_us(&self) -> bool
	{
		tools::get_bit_at(self.value, 2)
	}
	pub fn set_us(&mut self, value: bool)
	{
		tools::set_bit(&mut self.value, value, 2);
	}

	// Bit 1
	// The 'Read/Write' permissions flag. If the bit is set, the page is 
	// read/write. Otherwise when it is not set, the page is read-only. 
	// The WP bit in CR0 determines if this is only applied to userland, 
	// always giving the kernel write access (the default) or both userland and
	// the kernel (see Intel Manuals 3A 2-20).
	pub fn get_rw(&self) -> bool
	{
		tools::get_bit_at(self.value, 1)
	}
	pub fn set_rw(&mut self, value: bool)
	{
		tools::set_bit(&mut self.value, value, 1);
	}

	// Bit 0
	// If the bit is set, the page is actually in physical memory at the moment.
	// For example, when a page is swapped out, it is not in physical memory and
	// therefore not 'Present'.
	// If a page is called, but not present, a page fault will occur, 
	// and the OS should handle it.
	pub fn get_present(&self) -> bool
	{
		tools::get_bit_at(self.value, 0)
	}
	pub fn set_present(&mut self, value: bool)
	{
		tools::set_bit(&mut self.value, value, 0);
	}
}
