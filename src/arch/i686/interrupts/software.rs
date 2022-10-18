use super::State;

use core::slice;

#[derive(Copy, Clone)]
pub struct Syscall
{
	name: &'static str,
	pub handler: unsafe fn(u32, u32, u32)
}

pub static SYSCALLS: [Syscall; 2] =
[
	Syscall {name: "read", handler: sys_dummy},
	Syscall {name: "write", handler: sys_write}
];


pub unsafe fn handler(state: &State)
{
	syscall(state.eax, state.ebx, state.ecx, state.edx);
}

unsafe fn syscall(syscall_number: u32, arg1: u32, arg2: u32, arg3: u32)
{
	if (syscall_number as usize) < SYSCALLS.len()
	{
		let syscall = SYSCALLS[syscall_number as usize];
		crate::serial_println!("Called syscall {}({}, {}, {})", syscall.name, arg1, arg2, arg3);
		(syscall.handler)(arg1, arg2, arg3);
	}
	else
	{
		panic!("Invalid syscall {}({}, {}, {})", syscall_number, arg1, arg2, arg3);
	}
}

unsafe fn sys_dummy(_arg1: u32, _arg2: u32, _arg3: u32)
{

}

unsafe fn sys_write(_file_descriptor: u32, buffer: u32, len: u32)
{
	let len = len as usize;
	let buffer = slice::from_raw_parts(buffer as *const u8, len);
	for i in 0..len
	{
		crate::log!("{}", buffer[i] as char);
	}
}
