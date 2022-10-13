use crate::arch::port::{inb, outb, io_wait};

const ICW1_ICW4: u8 = 0x01;			/* ICW4 (not) needed */
const ICW1_SINGLE: u8 = 0x02;		/* Single (cascade) mode */
const ICW1_INTERVAL4: u8 = 0x04;	/* Call address interval 4 (8) */
const ICW1_LEVEL: u8 = 0x08;		/* Level triggered (edge) mode */
const ICW1_INIT: u8 = 0x10;			/* Initialization - required! */

const ICW4_8086: u8 = 0x01;			/* 8086/88 (MCS-80/85) mode */
const ICW4_AUTO: u8 = 0x02;			/* Auto (normal) EOI */
const ICW4_BUF_SLAVE: u8 = 0x08;	/* Buffered mode/slave */
const ICW4_BUF_MASTER: u8 = 0x0C;	/* Buffered mode/master */
const ICW4_SFNM: u8 = 0x10;			/* Special fully nested (not) */
 
const PIC_EOI: u8 = 0x20;			/* End-of-interrupt command code */

struct PIC
{
	id: u8,
	base: u32,
	command: u16,
	data: u16,
	mask: u8
}

impl PIC
{
	fn save_mask(&mut self)
	{
		self.mask = inb(self.data);
	}

	fn restore_mask(&self)
	{
		outb(self.data, self.mask);
	}

	fn init_with_offset(&self, offset: u8)
	{
		// ICW1
		outb(self.command, ICW1_INIT | ICW1_ICW4);
		io_wait();

		// ICW2
		outb(self.data, offset);
		io_wait();

		// ICW3
		if self.id == 1
		{
			outb(self.data, 0b0000_0100);
		}
		else if self.id == 2
		{
			outb(self.data, 0b0000_0010);
		}
		io_wait();

		// ICW4
		outb(self.data, ICW4_8086);
		io_wait();
	}

	fn remap(&mut self, offset: u8)
	{
		self.save_mask();
		self.init_with_offset(offset);
		self.restore_mask();
	}

	fn send_eoi(&self)
	{
		outb(self.command, PIC_EOI);
	}
}

static PIC1: PIC = PIC
{
	id: 1,
	base: 0x20,
	command: 0x20,
	data: 0x21,
	mask: 0
};

static PIC2: PIC = PIC
{
	id: 2,
	base: 0xa0,
	command: 0xa0,
	data: 0xa1,
	mask: 0
};

pub unsafe fn init()
{
	PIC1.init_with_offset(0x20);
	PIC2.init_with_offset(0x28);
}

pub unsafe fn send_eoi(irq: u8)
{
	if irq >= 8
	{
		PIC2.send_eoi();
	}
	PIC1.send_eoi();
}
