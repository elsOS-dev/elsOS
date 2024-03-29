pub mod initial;

#[cfg(target_arch = "x86")]
mod i686;

#[cfg(target_arch = "x86")]
pub use i686::
{
	halt,
	interrupts,
	port,
	rand,
	syscall
};
