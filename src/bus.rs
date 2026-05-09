use std::io::{self, Write};

use crate::error::CpuError;
use crate::memory::Memory;

const RAM_BASE: u64 = 0x8000_0000;
const UART_BASE: u64 = 0x1000_0000;
const UART_SIZE: u64 = 0x100;

pub struct Bus {
	memory: Memory,
	ram_size: usize,
}

impl Bus {
	pub fn new(ram_size: usize) -> Self {
		Self {
			memory: Memory::new(RAM_BASE, ram_size),
			ram_size,
		}
	}

	pub fn load(&self, addr: u64, width: u8) -> Result<u64, CpuError> {
		if in_range(addr, RAM_BASE, self.ram_size as u64) {
			return self.memory.load(addr, width);
		}

		if in_range(addr, UART_BASE, UART_SIZE) {
			return Err(CpuError::BusError(addr));
		}

		Err(CpuError::BusError(addr))
	}

	pub fn store(&mut self, addr: u64, width: u8, value: u64) -> Result<(), CpuError> {
		if in_range(addr, RAM_BASE, self.ram_size as u64) {
			return self.memory.store(addr, width, value);
		}

		if in_range(addr, UART_BASE, UART_SIZE) {
			return self.store_uart(addr, width, value);
		}

		Err(CpuError::BusError(addr))
	}

	fn store_uart(&mut self, addr: u64, width: u8, value: u64) -> Result<(), CpuError> {
		let offset = addr - UART_BASE;
		if offset != 0 || width != 1 {
			return Err(CpuError::BusError(addr));
		}

		let byte = (value & 0xff) as u8;
		let mut stdout = io::stdout();
		stdout.write_all(&[byte]).map_err(|_| CpuError::BusError(addr))?;
		stdout.flush().map_err(|_| CpuError::BusError(addr))?;

		Ok(())
	}

	pub fn load_bytes(&mut self, addr: u64, bytes: &[u8]) -> Result<(), CpuError> {
		for (offset, byte) in bytes.iter().copied().enumerate() {
			self.store(addr + offset as u64, 1, byte as u64)?;
		}

		Ok(())
	}
}

fn in_range(addr: u64, base: u64, size: u64) -> bool {
	addr >= base && addr < base + size
}
