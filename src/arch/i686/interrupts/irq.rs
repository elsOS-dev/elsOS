use super::State;
use super::pic;

const PIT_INTERRUPT: u8 = 0x00;

pub struct IRQ
{
	pub message: &'static str,
	pub handler: unsafe fn(&State)
}

pub static IRQS: [IRQ; 1] =
[
	IRQ {message: "Programmable Interrupt Timer Interrupt", handler: pit_interrupt}
];

pub unsafe fn handler(state: &State)
{
	(IRQS[(state.interrupt- 0x20) as usize].handler)(state);
	pic::send_eoi((state.interrupt - 0x20) as u8);
}

unsafe fn pit_interrupt(_state: &State)
{
	crate::time::JIFFIES += 1;
}
