use crate::error::CpuError;

pub struct Memory {
	base: u64,
	data: Vec<u8>,
}

impl Memory {
	pub fn new(base: u64, size: usize) -> Self {
		Self {
			base,
			data: vec![0; size],
		}
	}

	pub fn load(&self, addr: u64, width: u8) -> Result<u64, CpuError> {
		let offset = self.check_access(addr, width)?;
		let mut value = 0u64;
		let width = width as usize;

		for i in 0..width {
			value |= (self.data[offset + i] as u64) << (8 * i as u64);
		}

		Ok(value)
	}

	pub fn store(&mut self, addr: u64, width: u8, value: u64) -> Result<(), CpuError> {
		let offset = self.check_access(addr, width)?;
		let width = width as usize;

		for i in 0..width {
			self.data[offset + i] = ((value >> (8 * i as u64)) & 0xff) as u8;
		}

		Ok(())
	}

	fn check_access(&self, addr: u64, width: u8) -> Result<usize, CpuError> {
		match width {
			1 | 2 | 4 | 8 => {}
			_ => return Err(CpuError::BusError(addr)),
		}

		if width > 1 && !addr.is_multiple_of(width as u64) {
			return Err(CpuError::Misaligned { addr, width });
		}

		let offset = match addr.checked_sub(self.base) {
			Some(offset) => offset,
			None => return Err(CpuError::BusError(addr)),
		};
		let width = width as u64;
		let end = match offset.checked_add(width) {
			Some(end) => end,
			None => return Err(CpuError::BusError(addr)),
		};

		if end > self.data.len() as u64 {
			return Err(CpuError::BusError(addr));
		}

		Ok(offset as usize)
	}
}
