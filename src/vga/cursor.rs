use crate::utilities::{inb, outb};
use crate::vga::BUFFER_WIDTH;

const CRT_ADDR_REG: u32 = 0x3D4;
const CRT_DATA_REG: u32 = 0x3D5;

const CURSOR_START_REG: u8 = 0x0A;
const CURSOR_END_REG: u8 = 0x0B;

const CURSOR_HIGH_REG: u8 = 0x0E;
const CURSOR_LOW_REG: u8 = 0x0F;

pub fn init(cursor_start: u8, cursor_end: u8)
{
	outb(CRT_ADDR_REG, CURSOR_START_REG);
	outb(CRT_DATA_REG, (inb(CRT_DATA_REG) & 0xC0) | cursor_start);

	outb(CRT_ADDR_REG, CURSOR_END_REG);
	outb(CRT_DATA_REG, (inb(CRT_DATA_REG) & 0xE0) | cursor_end);
}

pub fn move_to(x: u16, y: u16)
{
	let pos: u16 = y * BUFFER_WIDTH as u16 + x;

	outb(CRT_ADDR_REG, CURSOR_LOW_REG);
	outb(CRT_DATA_REG, pos as u8);
	outb(CRT_ADDR_REG, CURSOR_HIGH_REG);
	outb(CRT_DATA_REG, (pos >> 8) as u8);
}
