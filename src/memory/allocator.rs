use alloc::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use super::*;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator
{
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8
	{
		//crate::serial_println!("trying to allocate {} ({:#x}) bytes...", _layout.size(), _layout.size());
        let address = vmalloc(_layout.size());
		//crate::serial_println!("allocated {} bytes at {:p}", _layout.size(), address);
		address as *mut u8
    }

	unsafe fn alloc_zeroed(&self, _layout: Layout) -> *mut u8
	{
		kzalloc(_layout.size()) as *mut u8
	}

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout)
	{
		//crate::serial_println!("deallocating {:p}", _ptr);
		vfree(_ptr as *mut c_void);
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
