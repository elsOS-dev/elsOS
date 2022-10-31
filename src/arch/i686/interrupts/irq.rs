use super::State;
use super::pic;

pub struct IRQ
{
	pub message: &'static str,
	pub handler: unsafe fn(&State)
}

pub static IRQS: [IRQ; 15] =
[
	IRQ {message: "Programmable Interrupt Timer Interrupt", handler: pit_interrupt},
	IRQ {message: "Keyboard Interrupt", handler: keyboard_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt},
	IRQ {message: "", handler: unhandled_interrupt}
];

pub unsafe fn handler(state: &State)
{
	let irq = state.interrupt - 0x20;
	(IRQS[irq as usize].handler)(state);
	pic::send_eoi(irq as u8);
}

unsafe fn pit_interrupt(_state: &State)
{
	crate::time::JIFFIES += 1;
}

unsafe fn keyboard_interrupt(_state: &State)
{
	crate::keyboard::get_scancode();
}

unsafe fn unhandled_interrupt(state: &State)
{
	let interrupt = state.interrupt;
	crate::serial_println!("Got unhandled irq {:02x}", interrupt);
}
