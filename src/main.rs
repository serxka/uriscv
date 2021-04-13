#![no_main]
#![no_std]

#![feature(asm, custom_test_frameworks, global_asm, panic_info_message)]
#![reexport_test_harness_main = "test_kmain"]
#![test_runner(crate::test::runner)]

global_asm!(include_str!("asm/boot.asm"));
global_asm!(include_str!("asm/extern.asm"));

#[macro_use]
mod util;
mod uart;

pub use util::abort;

#[no_mangle]
extern "C" fn kinit() {
	uart::Uart::new(0x1000_0000).init();

	#[cfg(test)]
	test_kmain();

	kmain();
}

fn kmain() {
	println!("hello world!");
}

#[cfg(test)]
mod test {
	use super::*;

	#[test_case]
	fn example_test_pass() {
		assert_eq!(1, 1);
	}

	#[test_case]
	fn example_test_fail() {
		assert_eq!(1, 2);
	}

	const EXIT_FAILURE: u32 = 0x13333;
	const EXIT_SUCCESS: u32 = 0x5555;
	const EXIT_RESET: u32 = 0x7777;

	pub fn exit_qemu(code: u32) -> ! {
		const EXIT_ADDR: u32 = 0x100000;
		let code = match code {
			EXIT_FAILURE | EXIT_SUCCESS | EXIT_RESET => code,
			_ => code << 16 | 0x3333,
		};

		unsafe {
			// store word into mm device address
			asm!("sw {0}, 0({1})", in(reg)code, in(reg)EXIT_ADDR);
			// if we fail then loop
			abort()
		}
	}

	pub trait Testable {
		fn run(&self);
	}

	impl<T> Testable for T
	where
		T: Fn(),
	{
		fn run(&self) {
			print!("{}...\t", core::any::type_name::<T>());
			self();
			println!("[passed]");
		}
	}

	pub fn runner(tests: &[&dyn Testable]) {
		println!("[[ running {} tests ]]", tests.len());
		for (i, test) in tests.iter().enumerate() {
			print!("[{}/{}] ", i + 1, tests.len());
			test.run();
		}
		exit_qemu(EXIT_SUCCESS);
	}

	#[panic_handler]
	fn panic(info: &core::panic::PanicInfo) -> ! {
		println!("[failed]\n");
		println!("error: {}", info);
		exit_qemu(EXIT_FAILURE)
	}
}
