use crate::error::CpuError;
use crate::isa::{AnyInstruction, CsrInstruction, Instruction};

#[inline]
fn bits(raw: u32, hi: u8, lo: u8) -> u32 {
	(raw >> lo) & ((1u32 << (hi - lo + 1)) - 1)
}

#[inline]
fn rd(raw: u32) -> u8 {
	bits(raw, 11, 7) as u8
}

#[inline]
fn rs1(raw: u32) -> u8 {
	bits(raw, 19, 15) as u8
}

#[inline]
fn rs2(raw: u32) -> u8 {
	bits(raw, 24, 20) as u8
}

#[inline]
fn funct3(raw: u32) -> u8 {
	bits(raw, 14, 12) as u8
}

#[inline]
fn funct7(raw: u32) -> u8 {
	bits(raw, 31, 25) as u8
}

#[inline]
fn sign_extend(value: u32, bits: u8) -> i64 {
	let shift = 64u32 - bits as u32;
	((value as i64) << shift) >> shift
}

#[inline]
fn imm_i(raw: u32) -> i64 {
	sign_extend(bits(raw, 31, 20), 12)
}

#[inline]
fn imm_s(raw: u32) -> i64 {
	let high = bits(raw, 31, 25) << 5;
	let low = bits(raw, 11, 7);
	sign_extend(high | low, 12)
}

#[inline]
fn imm_b(raw: u32) -> i64 {
	let bit12 = bits(raw, 31, 31) << 12;
	let bit11 = bits(raw, 7, 7) << 11;
	let bits10_5 = bits(raw, 30, 25) << 5;
	let bits4_1 = bits(raw, 11, 8) << 1;
	sign_extend(bit12 | bit11 | bits10_5 | bits4_1, 13)
}

#[inline]
fn imm_u(raw: u32) -> i64 {
	let imm = bits(raw, 31, 12) << 12;
	sign_extend(imm, 32)
}

#[inline]
fn imm_j(raw: u32) -> i64 {
	let bit20 = bits(raw, 31, 31) << 20;
	let bits19_12 = bits(raw, 19, 12) << 12;
	let bit11 = bits(raw, 20, 20) << 11;
	let bits10_1 = bits(raw, 30, 21) << 1;
	sign_extend(bit20 | bits19_12 | bit11 | bits10_1, 21)
}

pub fn decode(raw: u32) -> Result<AnyInstruction, CpuError> {
	let opcode = bits(raw, 6, 0);
	let decoded = match opcode {
		0b0110111 => AnyInstruction::Base(Instruction::Lui {
			rd: rd(raw),
			imm: imm_u(raw),
		}),
		0b0010111 => AnyInstruction::Base(Instruction::Auipc {
			rd: rd(raw),
			imm: imm_u(raw),
		}),
		0b1101111 => AnyInstruction::Base(Instruction::Jal {
			rd: rd(raw),
			imm: imm_j(raw),
		}),
		0b1100111 => {
			if funct3(raw) != 0b000 {
				return Err(CpuError::IllegalInstruction(raw));
			}
			AnyInstruction::Base(Instruction::Jalr {
				rd: rd(raw),
				rs1: rs1(raw),
				imm: imm_i(raw),
			})
		}
		0b1100011 => {
			let inst = match funct3(raw) {
				0b000 => Instruction::Beq {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_b(raw),
				},
				0b001 => Instruction::Bne {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_b(raw),
				},
				0b100 => Instruction::Blt {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_b(raw),
				},
				0b101 => Instruction::Bge {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_b(raw),
				},
				0b110 => Instruction::Bltu {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_b(raw),
				},
				0b111 => Instruction::Bgeu {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_b(raw),
				},
				_ => return Err(CpuError::IllegalInstruction(raw)),
			};
			AnyInstruction::Base(inst)
		}
		0b0000011 => {
			let inst = match funct3(raw) {
				0b000 => Instruction::Lb {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b001 => Instruction::Lh {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b010 => Instruction::Lw {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b011 => Instruction::Ld {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b100 => Instruction::Lbu {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b101 => Instruction::Lhu {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b110 => Instruction::Lwu {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				_ => return Err(CpuError::IllegalInstruction(raw)),
			};
			AnyInstruction::Base(inst)
		}
		0b0100011 => {
			let inst = match funct3(raw) {
				0b000 => Instruction::Sb {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_s(raw),
				},
				0b001 => Instruction::Sh {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_s(raw),
				},
				0b010 => Instruction::Sw {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_s(raw),
				},
				0b011 => Instruction::Sd {
					rs1: rs1(raw),
					rs2: rs2(raw),
					imm: imm_s(raw),
				},
				_ => return Err(CpuError::IllegalInstruction(raw)),
			};
			AnyInstruction::Base(inst)
		}
		0b0010011 => {
			let inst = match funct3(raw) {
				0b000 => Instruction::Addi {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b010 => Instruction::Slti {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b011 => Instruction::Sltiu {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b100 => Instruction::Xori {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b110 => Instruction::Ori {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b111 => Instruction::Andi {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b001 => {
					let shamt = bits(raw, 25, 20) as u8;
					if funct7(raw) != 0b0000000 {
						return Err(CpuError::IllegalInstruction(raw));
					}
					Instruction::Slli {
						rd: rd(raw),
						rs1: rs1(raw),
						shamt,
					}
				}
				0b101 => {
					let shamt = bits(raw, 25, 20) as u8;
					match funct7(raw) {
						0b0000000 => Instruction::Srli {
							rd: rd(raw),
							rs1: rs1(raw),
							shamt,
						},
						0b0100000 => Instruction::Srai {
							rd: rd(raw),
							rs1: rs1(raw),
							shamt,
						},
						_ => return Err(CpuError::IllegalInstruction(raw)),
					}
				}
				_ => return Err(CpuError::IllegalInstruction(raw)),
			};
			AnyInstruction::Base(inst)
		}
		0b0110011 => {
			let inst = match (funct3(raw), funct7(raw)) {
				(0b000, 0b0000000) => Instruction::Add {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b000, 0b0100000) => Instruction::Sub {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b001, 0b0000000) => Instruction::Sll {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b010, 0b0000000) => Instruction::Slt {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b011, 0b0000000) => Instruction::Sltu {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b100, 0b0000000) => Instruction::Xor {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b101, 0b0000000) => Instruction::Srl {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b101, 0b0100000) => Instruction::Sra {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b110, 0b0000000) => Instruction::Or {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b111, 0b0000000) => Instruction::And {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				_ => return Err(CpuError::IllegalInstruction(raw)),
			};
			AnyInstruction::Base(inst)
		}
		0b0011011 => {
			let inst = match funct3(raw) {
				0b000 => Instruction::Addiw {
					rd: rd(raw),
					rs1: rs1(raw),
					imm: imm_i(raw),
				},
				0b001 => {
					if funct7(raw) != 0b0000000 {
						return Err(CpuError::IllegalInstruction(raw));
					}
					Instruction::Slliw {
						rd: rd(raw),
						rs1: rs1(raw),
						shamt: bits(raw, 24, 20) as u8,
					}
				}
				0b101 => match funct7(raw) {
					0b0000000 => Instruction::Srliw {
						rd: rd(raw),
						rs1: rs1(raw),
						shamt: bits(raw, 24, 20) as u8,
					},
					0b0100000 => Instruction::Sraiw {
						rd: rd(raw),
						rs1: rs1(raw),
						shamt: bits(raw, 24, 20) as u8,
					},
					_ => return Err(CpuError::IllegalInstruction(raw)),
				},
				_ => return Err(CpuError::IllegalInstruction(raw)),
			};
			AnyInstruction::Base(inst)
		}
		0b0111011 => {
			let inst = match (funct3(raw), funct7(raw)) {
				(0b000, 0b0000000) => Instruction::Addw {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b000, 0b0100000) => Instruction::Subw {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b001, 0b0000000) => Instruction::Sllw {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b101, 0b0000000) => Instruction::Srlw {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				(0b101, 0b0100000) => Instruction::Sraw {
					rd: rd(raw),
					rs1: rs1(raw),
					rs2: rs2(raw),
				},
				_ => return Err(CpuError::IllegalInstruction(raw)),
			};
			AnyInstruction::Base(inst)
		}
		0b0001111 => {
			let inst = match funct3(raw) {
				0b000 => Instruction::Fence {
					pred: bits(raw, 27, 24) as u8,
					succ: bits(raw, 23, 20) as u8,
				},
				0b001 => Instruction::FenceI,
				_ => return Err(CpuError::IllegalInstruction(raw)),
			};
			AnyInstruction::Base(inst)
		}
		0b1110011 => {
			let f3 = funct3(raw);
			if f3 == 0b000 {
				match imm_i(raw) {
					0 => AnyInstruction::Base(Instruction::Ecall),
					1 => AnyInstruction::Base(Instruction::Ebreak),
					_ => return Err(CpuError::IllegalInstruction(raw)),
				}
			} else {
				let csr = bits(raw, 31, 20) as u16;
				let inst = match f3 {
					0b001 => CsrInstruction::Csrrw {
						rd: rd(raw),
						rs1_or_uimm: rs1(raw),
						csr,
					},
					0b010 => CsrInstruction::Csrrs {
						rd: rd(raw),
						rs1_or_uimm: rs1(raw),
						csr,
					},
					0b011 => CsrInstruction::Csrrc {
						rd: rd(raw),
						rs1_or_uimm: rs1(raw),
						csr,
					},
					0b101 => CsrInstruction::Csrrwi {
						rd: rd(raw),
						rs1_or_uimm: rs1(raw),
						csr,
					},
					0b110 => CsrInstruction::Csrrsi {
						rd: rd(raw),
						rs1_or_uimm: rs1(raw),
						csr,
					},
					0b111 => CsrInstruction::Csrrci {
						rd: rd(raw),
						rs1_or_uimm: rs1(raw),
						csr,
					},
					_ => return Err(CpuError::IllegalInstruction(raw)),
				};
				AnyInstruction::Csr(inst)
			}
		}
		_ => return Err(CpuError::IllegalInstruction(raw)),
	};

	Ok(decoded)
}
