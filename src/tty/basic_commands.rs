use crate::utilities;

pub fn execute(command: &str)
{
	match command
	{
		"halt" | "exit" => halt(),
		"help" => help(),
		_ => crate::println!("{}: unknown command. Use help for more", command)
	};
}

fn halt()
{
	utilities::shutdown_qemu();
}

fn help()
{
	crate::println!("Supported commands:");
	crate::println!("halt | exit: stop the virtual machine (qemu only)");
	crate::println!("help: show this help");
}
