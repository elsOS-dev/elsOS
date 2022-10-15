pub mod gdt;
pub mod port;

pub fn init()
{
	gdt::init();
}
