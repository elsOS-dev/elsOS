use crate::arch::i686::instructions;

mod exceptions;
pub mod idt;
mod irq;
mod pic;
mod software;

#[inline(always)]
pub unsafe fn init()
{
	idt::init();
	idt::load();
	pic::init();
}

#[inline(always)]
pub unsafe fn enable()
{
	instructions::sti();
}
#[inline(always)]
pub unsafe fn disable()
{
	instructions::cli();
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct State
{
	pub ds: u32,
	// pusha
	pub edi: u32,
	pub esi: u32,
	pub ebp: u32,
	pub esp: u32,
	pub ebx: u32,
	pub edx: u32,
	pub ecx: u32,
	pub eax: u32,
	// push interrupt
	pub interrupt: u32,
	// pushed automatically
	pub error: u32,
	// pushed by the CPU
	pub eip: u32,
	pub cs: u32,
	pub eflags: u32,
}

impl State
{
	fn save(self)
	{
		unsafe
		{
			let state = &mut crate::INTERRUPT_STATE;
			state.eax = self.eax;
			state.ebx = self.ebx;
			state.ecx = self.ecx;
			state.edx = self.edx;

			state.esi = self.esi;
			state.edi = self.edi;
			state.esp = self.esp;
			state.ebp = self.ebp;

			state.cs = self.cs;
			state.ds = self.ds;

			state.interrupt = self.interrupt;
			state.error = self.error;

			state.eip = self.eip;
			state.eflags = self.eflags;
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn interrupt_handler(state: &State) -> usize
{
	let interrupt = state.interrupt;
	match interrupt
	{
		0x00..=0x1f =>
		{
			exceptions::handler(state);
			0
		},
		0x20..=0x2f =>
		{
			irq::handler(state);
			0
		}
		0x30 | 0x80 =>
		{
			software::handler(state)
		}
		_ =>
		{
			crate::serial_println!("Got unhandled interrupt {:02x}", interrupt);
			0
		}
	}
}
