use core::ffi::c_void;

extern "C"
{
	pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	pub fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	pub fn memset(dest: *mut c_void, val: usize, len: usize) -> *mut c_void;
}
