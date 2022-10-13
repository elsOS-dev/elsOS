pub mod initial;

#[cfg(target_arch = "x86")]
mod i686;

#[cfg(target_arch = "x86")]
pub use i686::interrupts;

#[cfg(target_arch = "x86")]
pub use i686::port;
