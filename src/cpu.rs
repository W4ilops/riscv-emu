use std::collections::HashMap;

use crate::bus::Bus;
use crate::decode::decode;
use crate::error::CpuError;
use crate::execute::execute;

pub struct Cpu {
	pub(crate) registers: [u64; 32],
	pub(crate) pc: u64,
	pub(crate) csrs: HashMap<u16, u64>,
	pub(crate) bus: Bus,
}

impl Cpu {
	pub fn new(entry_point: u64, bus: Bus) -> Self {
		Self {
			registers: [0; 32],
			pc: entry_point,
			csrs: HashMap::new(),
			bus,
		}
	}

	pub fn read_reg(&self, r: u8) -> u64 {
		if r == 0 {
			0
		} else {
			self.registers[r as usize]
		}
	}

	pub fn write_reg(&mut self, r: u8, val: u64) {
		if r != 0 {
			self.registers[r as usize] = val;
		}
	}

	pub fn read_csr(&self, addr: u16) -> Result<u64, CpuError> {
		Ok(*self.csrs.get(&addr).unwrap_or(&0))
	}

	pub fn write_csr(&mut self, addr: u16, val: u64) -> Result<(), CpuError> {
		self.csrs.insert(addr, val);
		Ok(())
	}

	pub fn step(&mut self) -> Result<(), CpuError> {
		let raw = self.bus.load(self.pc, 4)? as u32;
		let instr = decode(raw)?;
		execute(instr, self)
	}

	pub fn run(&mut self) -> Result<(), CpuError> {
		loop {
			match self.step() {
				Ok(()) => {}
				Err(CpuError::Ebreak) => return Ok(()),
				Err(err) => return Err(err),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Cpu;
	use crate::bus::Bus;
	use rstest::rstest;

	const RAM_BASE: u64 = 0x8000_0000;
	const EBREAK: u32 = 0x0010_0073;

	fn run_program(raw: &[u32]) -> Cpu {
		let mut bus = Bus::new(0x2000);
		let mut bytes = Vec::with_capacity(raw.len() * 4);
		for word in raw {
			bytes.extend_from_slice(&word.to_le_bytes());
		}
		bus.load_bytes(RAM_BASE, &bytes).unwrap();
		let mut cpu = Cpu::new(RAM_BASE, bus);
		cpu.run().unwrap();
		cpu
	}

	fn encode_r(funct7: u32, rs2: u8, rs1: u8, funct3: u32, rd: u8, opcode: u32) -> u32 {
		(funct7 << 25)
			| ((rs2 as u32) << 20)
			| ((rs1 as u32) << 15)
			| (funct3 << 12)
			| ((rd as u32) << 7)
			| opcode
	}

	fn encode_i(imm: i32, rs1: u8, funct3: u32, rd: u8, opcode: u32) -> u32 {
		let imm = (imm as u32) & 0x0fff;
		(imm << 20)
			| ((rs1 as u32) << 15)
			| (funct3 << 12)
			| ((rd as u32) << 7)
			| opcode
	}

	fn encode_s(imm: i32, rs2: u8, rs1: u8, funct3: u32, opcode: u32) -> u32 {
		let imm = (imm as u32) & 0x0fff;
		let imm_11_5 = (imm >> 5) & 0x7f;
		let imm_4_0 = imm & 0x1f;
		(imm_11_5 << 25)
			| ((rs2 as u32) << 20)
			| ((rs1 as u32) << 15)
			| (funct3 << 12)
			| (imm_4_0 << 7)
			| opcode
	}

	fn encode_b(imm: i32, rs2: u8, rs1: u8, funct3: u32, opcode: u32) -> u32 {
		let imm = (imm as u32) & 0x1fff;
		let bit12 = (imm >> 12) & 0x1;
		let bit11 = (imm >> 11) & 0x1;
		let bits10_5 = (imm >> 5) & 0x3f;
		let bits4_1 = (imm >> 1) & 0x0f;
		(bit12 << 31)
			| (bits10_5 << 25)
			| ((rs2 as u32) << 20)
			| ((rs1 as u32) << 15)
			| (funct3 << 12)
			| (bits4_1 << 8)
			| (bit11 << 7)
			| opcode
	}

	fn encode_u(imm: u32, rd: u8, opcode: u32) -> u32 {
		(imm & 0xffff_f000) | ((rd as u32) << 7) | opcode
	}

	fn encode_j(imm: i32, rd: u8, opcode: u32) -> u32 {
		let imm = (imm as u32) & 0x1f_ffff;
		let bit20 = (imm >> 20) & 0x1;
		let bits10_1 = (imm >> 1) & 0x3ff;
		let bit11 = (imm >> 11) & 0x1;
		let bits19_12 = (imm >> 12) & 0xff;
		(bit20 << 31)
			| (bits19_12 << 12)
			| (bit11 << 20)
			| (bits10_1 << 21)
			| ((rd as u32) << 7)
			| opcode
	}

	#[rstest]
	#[case(5, 7, 12)]
	#[case(10, -3, 7)]
	#[case(-5, -6, -11)]
	fn test_addi(#[case] rs1: i32, #[case] imm: i32, #[case] expected: i32) {
		let program = [
			encode_i(rs1, 0, 0b000, 1, 0b001_0011),
			encode_i(imm, 1, 0b000, 2, 0b001_0011),
			EBREAK,
		];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(2), expected as i64 as u64);
	}

	#[rstest]
	#[case(3, 4, 7)]
	#[case(-8, 2, -6)]
	fn test_add(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
		let program = [
			encode_i(a, 0, 0b000, 1, 0b001_0011),
			encode_i(b, 0, 0b000, 2, 0b001_0011),
			encode_r(0b0000000, 2, 1, 0b000, 3, 0b011_0011),
			EBREAK,
		];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(3), expected as i64 as u64);
	}

	#[rstest]
	#[case(10, 4, 6)]
	#[case(-3, -7, 4)]
	fn test_sub(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
		let program = [
			encode_i(a, 0, 0b000, 1, 0b001_0011),
			encode_i(b, 0, 0b000, 2, 0b001_0011),
			encode_r(0b0100000, 2, 1, 0b000, 3, 0b011_0011),
			EBREAK,
		];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(3), expected as i64 as u64);
	}

	#[test]
	fn test_lui() {
		let program = [encode_u(0x1234_5000, 1, 0b011_0111), EBREAK];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(1), 0x1234_5000);
	}

	#[test]
	fn test_auipc() {
		let program = [encode_u(0x0000_1000, 1, 0b001_0111), EBREAK];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(1), RAM_BASE + 0x1000);
	}

	#[test]
	fn test_jal() {
		let program = [
			encode_j(8, 1, 0b110_1111),
			encode_i(1, 0, 0b000, 2, 0b001_0011),
			encode_i(2, 0, 0b000, 2, 0b001_0011),
			EBREAK,
		];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(1), RAM_BASE + 4);
		assert_eq!(cpu.read_reg(2), 2);
	}

	#[test]
	fn test_jalr() {
		let program = [
			encode_u(0x0000_0000, 3, 0b001_0111),
			encode_i(16, 3, 0b000, 3, 0b001_0011),
			encode_i(0, 3, 0b000, 1, 0b110_0111),
			encode_i(1, 0, 0b000, 2, 0b001_0011),
			encode_i(9, 0, 0b000, 2, 0b001_0011),
			EBREAK,
		];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(1), RAM_BASE + 12);
		assert_eq!(cpu.read_reg(2), 9);
	}

	#[rstest]
	#[case(true, 2)]
	#[case(false, 7)]
	fn test_beq(#[case] taken: bool, #[case] expected: u64) {
		let rs2_value = if taken { 1 } else { 2 };
		let program = [
			encode_i(1, 0, 0b000, 1, 0b001_0011),
			encode_i(rs2_value, 0, 0b000, 2, 0b001_0011),
			encode_b(12, 2, 1, 0b000, 0b110_0011),
			encode_i(7, 0, 0b000, 3, 0b001_0011),
			EBREAK,
			encode_i(2, 0, 0b000, 3, 0b001_0011),
			EBREAK,
		];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(3), expected);
	}

	#[test]
	fn test_lw_sw() {
		let program = [
			encode_u(0x0000_0000, 1, 0b001_0111),
			encode_i(0x100, 1, 0b000, 1, 0b001_0011),
			encode_i(0x7b, 0, 0b000, 2, 0b001_0011),
			encode_s(0, 2, 1, 0b010, 0b010_0011),
			encode_i(0, 1, 0b010, 3, 0b000_0011),
			EBREAK,
		];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(3), 0x7b);
	}

	#[test]
	fn test_ld_sd() {
		let program = [
			encode_u(0x0000_0000, 1, 0b001_0111),
			encode_i(0x120, 1, 0b000, 1, 0b001_0011),
			encode_u(0x1234_5000, 2, 0b011_0111),
			encode_i(0x678, 2, 0b000, 2, 0b001_0011),
			encode_s(0, 2, 1, 0b011, 0b010_0011),
			encode_i(0, 1, 0b011, 3, 0b000_0011),
			EBREAK,
		];
		let cpu = run_program(&program);
		assert_eq!(cpu.read_reg(3), 0x1234_5678);
	}
}
