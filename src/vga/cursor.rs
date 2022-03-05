use crate::tools::{inb, outb};
use crate::vga::BUFFER_WIDTH;

const CRT_ADDR_REG: u32 = 0x3D4;
const CRT_DATA_REG: u32 = 0x3D5;

const CURSOR_START_REG: u8 = 0x0A;
const CURSOR_END_REG: u8 = 0x0B;

const CURSOR_HIGH_REG: u8 = 0x0E;
const CURSOR_LOW_REG: u8 = 0x0F;

pub struct Cursor
{
	pub offset: usize
}

impl Cursor
{
	pub fn init(cursor_start: u8, cursor_end: u8)
	{
		outb(CRT_ADDR_REG, CURSOR_START_REG);
		outb(CRT_DATA_REG, (inb(CRT_DATA_REG) & 0xC0) | cursor_start);
	
		outb(CRT_ADDR_REG, CURSOR_END_REG);
		outb(CRT_DATA_REG, (inb(CRT_DATA_REG) & 0xE0) | cursor_end);
	}
	
	pub fn disable()
	{
		outb(CRT_ADDR_REG, 0x0A);
		outb(CRT_DATA_REG, 0x20);
	}
	
	pub fn move_to(x: u16, y: u16)
	{
		let mut pos: u16 = y * BUFFER_WIDTH as u16 + x;

		unsafe
		{
			if CURSOR.offset as u16 <= pos
			{
				pos -= CURSOR.offset as u16;
			}
		}
		outb(CRT_ADDR_REG, CURSOR_LOW_REG);
		outb(CRT_DATA_REG, pos as u8);
		outb(CRT_ADDR_REG, CURSOR_HIGH_REG);
		outb(CRT_DATA_REG, (pos >> 8) as u8);
	}
	
	pub fn get_position() -> (u16, u16)
	{
		let mut pos: u16;
	
		outb(CRT_ADDR_REG, CURSOR_LOW_REG);
		pos = inb(CRT_DATA_REG) as u16;
		outb(CRT_ADDR_REG, CURSOR_HIGH_REG);
		pos |= (inb(CRT_DATA_REG) as u16) << 8;
		unsafe
		{
			pos += CURSOR.offset as u16;
		}
		(pos % BUFFER_WIDTH as u16, pos / BUFFER_WIDTH as u16)
	}
}

pub static mut CURSOR: Cursor = Cursor
{
	offset: 0
};

