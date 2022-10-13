use super::State;

pub enum ExceptionClass
{
	Trap, Fault, Abort, NA
}

pub struct Exception
{
	pub message: &'static str,
	pub class: ExceptionClass,
	pub has_error: bool
}

pub static EXCEPTIONS: [Exception; 32] =
[
	Exception {message: "Divide Error Exception", class: ExceptionClass::Fault, has_error: false},
	Exception {message: "Debug Exception", class: ExceptionClass::Fault, has_error: false},
	Exception {message: "NMI Interrupt", class: ExceptionClass::NA, has_error: false},
	Exception {message: "Breakpoint Exception", class: ExceptionClass::Trap, has_error: false},
	Exception {message: "Overflow Exception", class: ExceptionClass::Trap, has_error: false},
	Exception {message: "Bound Range Exceeded Exception", class: ExceptionClass::Fault, has_error: false},
	Exception {message: "Invalid Opcode Exception", class: ExceptionClass::Fault, has_error: false},
	Exception {message: "Device Not Available Exception", class: ExceptionClass::Fault, has_error: false},
	Exception {message: "Double Fault Exception", class: ExceptionClass::Abort, has_error: true},
	Exception {message: "Coprocessor Segment Overrun", class: ExceptionClass::Abort, has_error: false},
	Exception {message: "Invalid TSS Exception", class: ExceptionClass::Fault, has_error: true},
	Exception {message: "Segment Not Present", class: ExceptionClass::Fault, has_error: true},
	Exception {message: "Stack Fault Exception", class: ExceptionClass::Fault, has_error: true},
	Exception {message: "General Protection Exception", class: ExceptionClass::Fault, has_error: true},
	Exception {message: "Page Fault Exception", class: ExceptionClass::Fault, has_error: true},
	Exception {message: "Reserved", class: ExceptionClass::NA, has_error: false},

	Exception {message: "x87 Floating-Point Exception", class: ExceptionClass::Fault, has_error: false},
	Exception {message: "Alignment Check Exception", class: ExceptionClass::Fault, has_error: true},
	Exception {message: "Machine Check Exception", class: ExceptionClass::Abort, has_error: false},
	Exception {message: "SIMD Floating-Point Exception", class: ExceptionClass::Fault, has_error: false},
	Exception {message: "Virtualization Exception", class: ExceptionClass::Fault, has_error: false},
	Exception {message: "Control Protection Exception", class: ExceptionClass::Fault, has_error: true},
	Exception {message: "Reserved", class: ExceptionClass::NA, has_error: false},
	Exception {message: "Reserved", class: ExceptionClass::NA, has_error: false},
	Exception {message: "Reserved", class: ExceptionClass::NA, has_error: false},
	Exception {message: "Reserved", class: ExceptionClass::NA, has_error: false},
	Exception {message: "Reserved", class: ExceptionClass::NA, has_error: false},
	Exception {message: "Reserved", class: ExceptionClass::NA, has_error: false},
	Exception {message: "Hypervisor Injection Exception", class: ExceptionClass::Fault, has_error: false},
	Exception {message: "VMM Communication Exception", class: ExceptionClass::Fault, has_error: true},
	Exception {message: "Security Exception", class: ExceptionClass::Fault, has_error: true},
	Exception {message: "Reserved", class: ExceptionClass::NA, has_error: false},
];

pub fn handler(state: &State)
{
	state.save();
	let interrupt = state.interrupt;
	let exception = &EXCEPTIONS[interrupt as usize];
	let error = state.error;
	if exception.has_error
	{
		panic!("{:02x} - {} - error {:08x}", interrupt, exception.message, error);
	}
	else
	{
		panic!("{:02x} - {}", interrupt, exception.message);
	}
}
