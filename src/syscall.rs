use crate::arch;

#[inline(always)]
pub unsafe fn syscall(syscall_number: u32, arg1: u32, arg2: u32, arg3: u32) -> usize
{
	arch::syscall::syscall(syscall_number, arg1, arg2, arg3)
}

#[inline(always)]
pub unsafe fn read(file_descriptor: u32, buffer: u32, len: u32) -> usize
{
	syscall(0, file_descriptor, buffer, len)
}

#[inline(always)]
pub unsafe fn write(file_descriptor: u32, buffer: u32, len: u32) -> usize
{
	syscall(1, file_descriptor, buffer, len)
}
