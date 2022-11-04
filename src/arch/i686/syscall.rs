use core::arch::asm;

pub unsafe fn syscall(syscall_number: u32, arg1: u32, arg2: u32, arg3: u32) -> usize
{
	let ret: usize;
	asm!("int 0x80; mov eax, retval",
			in("eax") syscall_number,
			in("ebx") arg1,
			in("ecx") arg2,
			in("edx") arg3,
			lateout("eax") ret);
	ret
}

