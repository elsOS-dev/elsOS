use core::mem::size_of;
use crate::arch::i686;

#[repr(C, packed)]
pub struct descriptor
{
	limit: u16,
	base: u32
}

enum GateType
{
	Task = 0b0101,
	Interrupt16 = 0b0110,
	Trap16 = 0b0111,
	Interrupt32 = 0b1110,
	Trap32 = 0b1111,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct gate
{
	isr_low: u16,
	segment: u16,
	reserved: u8,
	flags: u8,
	isr_high: u16
}

impl gate
{
	fn init(&mut self)
	{
		self.segment = 0x08;
	}

	fn set_isr(&mut self, isr: u32)
	{
		self.isr_low = (isr & 0xffff) as u16;
		self.isr_high = (isr >> 16) as u16;
	}

	fn set_present(&mut self)
	{
		self.set_flag(0b1000_0000);
	}

	fn unset_present(&mut self)
	{
		self.unset_flag(0b1000_0000);
	}

	fn set_type(&mut self, gate_type: GateType)
	{
		// unset the type represented by only 0s
		self.unset_flag(GateType::Trap32 as u8);
		self.set_flag(gate_type as u8);
	}

	#[inline(always)]
	fn set_flag(&mut self, flag: u8)
	{
		self.flags |= flag;
	}

	#[inline(always)]
	fn unset_flag(&mut self, flag: u8)
	{
		self.flags &= !flag;
	}
}

static mut DESCRIPTOR: descriptor = descriptor
{
	limit: 0,
	base: 0
};

static mut IDT: [gate; 33] = [gate
{
	isr_low: 0,
	segment: 0,
	reserved: 0,
	flags: 0,
	isr_high: 0
}; 33];

extern "C"
{
	static mut _isr_table: [u32; 33];
}

pub unsafe fn init()
{
	DESCRIPTOR.base = &IDT as *const _ as u32;
	DESCRIPTOR.limit = (size_of::<gate>() * IDT.len() - 1) as u16;

	for (vector, gate) in IDT.iter_mut().enumerate()
	{
		gate.init();
		gate.set_isr(_isr_table[vector as usize]);
		gate.set_type(GateType::Interrupt32);
		gate.set_present();
	}
}

pub unsafe fn load()
{
	i686::instructions::lidt(&DESCRIPTOR);
}
