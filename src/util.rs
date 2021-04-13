#[macro_export]
macro_rules! print {
	($($args:tt)+) => ({
		use core::fmt::Write;
		let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
	});
}

#[macro_export]
macro_rules! println {
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	println!("\n{}", info);
	abort();
}

pub fn abort() -> ! {
	unsafe {
		asm!("wfi", options(nomem, nostack));
		loop {}
	}
}
