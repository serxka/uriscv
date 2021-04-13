use core::fmt::{Error, Write};

pub struct Uart {
	device_addr: *mut u8,
}

impl Uart {
	pub fn new(device_addr: usize) -> Uart {
		Uart {
			device_addr: device_addr as *mut u8,
		}
	}

	pub fn init(&mut self) {
		let da = self.device_addr;
		unsafe {
			// Set word length
			let lcr = (1 << 0) | (1 << 1);
			da.add(3).write_volatile(lcr);
			// Enable FIFO
			da.add(2).write_volatile(1 << 0);
			// Enable receiver buffer int
			da.add(1).write_volatile(1 << 0);

			// Set UART to read new divisor
			da.add(3).write_volatile(lcr | 1 << 7);
			// Set divisor for buad rate
			let divisor = 592u16;
			da.add(0).write_volatile((divisor & 0xFF) as u8);
			da.add(1).write_volatile((divisor >> 8) as u8);
			// Disable writing of divisor
			da.add(3).write_volatile(lcr);
		}
	}

	#[allow(dead_code)]
	pub fn is_empty(&self) -> bool {
		let da = self.device_addr;
		unsafe { da.add(5).read_volatile() & 1 == 0 }
	}

	/// Blocking, use `is_data` to check if data is ready to be read
	#[allow(dead_code)]
	pub fn get(&self) -> u8 {
		let da = self.device_addr;
		loop {
			if !self.is_empty() {
				unsafe {
					return da.add(0).read_volatile();
				}
			}
		}
	}

	pub fn put(&mut self, d: u8) {
		let da = self.device_addr;
		unsafe {
			da.add(0).write_volatile(d);
		}
	}
}

impl Write for Uart {
	fn write_str(&mut self, s: &str) -> Result<(), Error> {
		for c in s.bytes() {
			self.put(c);
		}
		Ok(())
	}
}
