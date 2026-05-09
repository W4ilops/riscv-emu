use crate::cpu::Cpu;
use crate::error::CpuError;
use crate::isa::{AnyInstruction, CsrInstruction, Instruction};

pub fn execute(instr: AnyInstruction, cpu: &mut Cpu) -> Result<(), CpuError> {
	match instr {
		AnyInstruction::Base(inst) => execute_base(inst, cpu)?,
		AnyInstruction::Csr(inst) => execute_csr(inst, cpu)?,
	}

	Ok(())
}

fn execute_base(inst: Instruction, cpu: &mut Cpu) -> Result<(), CpuError> {
	match inst {
		Instruction::Add { rd, rs1, rs2 } => {
			let value = read_reg(cpu, rs1).wrapping_add(read_reg(cpu, rs2));
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Sub { rd, rs1, rs2 } => {
			let value = read_reg(cpu, rs1).wrapping_sub(read_reg(cpu, rs2));
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Sll { rd, rs1, rs2 } => {
			let shamt = (read_reg(cpu, rs2) & 0x3f) as u32;
			let value = read_reg(cpu, rs1) << shamt;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Slt { rd, rs1, rs2 } => {
			let value = (read_reg(cpu, rs1) as i64) < (read_reg(cpu, rs2) as i64);
			write_reg(cpu, rd, value as u64);
			advance_pc(cpu);
		}
		Instruction::Sltu { rd, rs1, rs2 } => {
			let value = read_reg(cpu, rs1) < read_reg(cpu, rs2);
			write_reg(cpu, rd, value as u64);
			advance_pc(cpu);
		}
		Instruction::Xor { rd, rs1, rs2 } => {
			let value = read_reg(cpu, rs1) ^ read_reg(cpu, rs2);
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Srl { rd, rs1, rs2 } => {
			let shamt = (read_reg(cpu, rs2) & 0x3f) as u32;
			let value = read_reg(cpu, rs1) >> shamt;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Sra { rd, rs1, rs2 } => {
			let shamt = (read_reg(cpu, rs2) & 0x3f) as u32;
			let value = ((read_reg(cpu, rs1) as i64) >> shamt) as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Or { rd, rs1, rs2 } => {
			let value = read_reg(cpu, rs1) | read_reg(cpu, rs2);
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::And { rd, rs1, rs2 } => {
			let value = read_reg(cpu, rs1) & read_reg(cpu, rs2);
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Addw { rd, rs1, rs2 } => {
			let value = ((read_reg(cpu, rs1) as u32).wrapping_add(read_reg(cpu, rs2) as u32) as i32)
				as i64 as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Subw { rd, rs1, rs2 } => {
			let value = ((read_reg(cpu, rs1) as u32).wrapping_sub(read_reg(cpu, rs2) as u32) as i32)
				as i64 as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Sllw { rd, rs1, rs2 } => {
			let shamt = (read_reg(cpu, rs2) & 0x1f) as u32;
			let value = ((read_reg(cpu, rs1) as u32) << shamt) as i32 as i64 as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Srlw { rd, rs1, rs2 } => {
			let shamt = (read_reg(cpu, rs2) & 0x1f) as u32;
			let value = ((read_reg(cpu, rs1) as u32) >> shamt) as i32 as i64 as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Sraw { rd, rs1, rs2 } => {
			let shamt = (read_reg(cpu, rs2) & 0x1f) as u32;
			let value = ((read_reg(cpu, rs1) as i32) >> shamt) as i64 as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Addi { rd, rs1, imm } => {
			let value = (read_reg(cpu, rs1) as i64).wrapping_add(imm) as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Slti { rd, rs1, imm } => {
			let value = (read_reg(cpu, rs1) as i64) < imm;
			write_reg(cpu, rd, value as u64);
			advance_pc(cpu);
		}
		Instruction::Sltiu { rd, rs1, imm } => {
			let value = read_reg(cpu, rs1) < imm as u64;
			write_reg(cpu, rd, value as u64);
			advance_pc(cpu);
		}
		Instruction::Xori { rd, rs1, imm } => {
			let value = read_reg(cpu, rs1) ^ imm as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Ori { rd, rs1, imm } => {
			let value = read_reg(cpu, rs1) | imm as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Andi { rd, rs1, imm } => {
			let value = read_reg(cpu, rs1) & imm as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Slli { rd, rs1, shamt } => {
			let shamt = (shamt & 0x3f) as u32;
			let value = read_reg(cpu, rs1) << shamt;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Srli { rd, rs1, shamt } => {
			let shamt = (shamt & 0x3f) as u32;
			let value = read_reg(cpu, rs1) >> shamt;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Srai { rd, rs1, shamt } => {
			let shamt = (shamt & 0x3f) as u32;
			let value = ((read_reg(cpu, rs1) as i64) >> shamt) as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Addiw { rd, rs1, imm } => {
			let value = ((read_reg(cpu, rs1) as u32).wrapping_add(imm as u32) as i32) as i64 as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Slliw { rd, rs1, shamt } => {
			let shamt = (shamt & 0x1f) as u32;
			let value = ((read_reg(cpu, rs1) as u32) << shamt) as i32 as i64 as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Srliw { rd, rs1, shamt } => {
			let shamt = (shamt & 0x1f) as u32;
			let value = ((read_reg(cpu, rs1) as u32) >> shamt) as i32 as i64 as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Sraiw { rd, rs1, shamt } => {
			let shamt = (shamt & 0x1f) as u32;
			let value = ((read_reg(cpu, rs1) as i32) >> shamt) as i64 as u64;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Lb { rd, rs1, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			let value = sign_extend(cpu.bus.load(addr, 1)?, 8);
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Lh { rd, rs1, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			let value = sign_extend(cpu.bus.load(addr, 2)?, 16);
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Lw { rd, rs1, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			let value = sign_extend(cpu.bus.load(addr, 4)?, 32);
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Ld { rd, rs1, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			let value = cpu.bus.load(addr, 8)?;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Lbu { rd, rs1, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			let value = cpu.bus.load(addr, 1)? & 0xff;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Lhu { rd, rs1, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			let value = cpu.bus.load(addr, 2)? & 0xffff;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Lwu { rd, rs1, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			let value = cpu.bus.load(addr, 4)? & 0xffff_ffff;
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Jalr { rd, rs1, imm } => {
			let link = cpu.pc.wrapping_add(4);
			let target = add_signed(read_reg(cpu, rs1), imm) & !1u64;
			write_reg(cpu, rd, link);
			cpu.pc = target;
		}
		Instruction::Fence { .. } => {
			advance_pc(cpu);
		}
		Instruction::FenceI => {
			advance_pc(cpu);
		}
		Instruction::Ecall => return Err(CpuError::Ecall),
		Instruction::Ebreak => return Err(CpuError::Ebreak),
		Instruction::Sb { rs1, rs2, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			cpu.bus.store(addr, 1, read_reg(cpu, rs2))?;
			advance_pc(cpu);
		}
		Instruction::Sh { rs1, rs2, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			cpu.bus.store(addr, 2, read_reg(cpu, rs2))?;
			advance_pc(cpu);
		}
		Instruction::Sw { rs1, rs2, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			cpu.bus.store(addr, 4, read_reg(cpu, rs2))?;
			advance_pc(cpu);
		}
		Instruction::Sd { rs1, rs2, imm } => {
			let addr = add_signed(read_reg(cpu, rs1), imm);
			cpu.bus.store(addr, 8, read_reg(cpu, rs2))?;
			advance_pc(cpu);
		}
		Instruction::Beq { rs1, rs2, imm } => {
			let taken = read_reg(cpu, rs1) == read_reg(cpu, rs2);
			branch(cpu, imm, taken);
		}
		Instruction::Bne { rs1, rs2, imm } => {
			let taken = read_reg(cpu, rs1) != read_reg(cpu, rs2);
			branch(cpu, imm, taken);
		}
		Instruction::Blt { rs1, rs2, imm } => {
			let taken = (read_reg(cpu, rs1) as i64) < (read_reg(cpu, rs2) as i64);
			branch(cpu, imm, taken);
		}
		Instruction::Bge { rs1, rs2, imm } => {
			let taken = (read_reg(cpu, rs1) as i64) >= (read_reg(cpu, rs2) as i64);
			branch(cpu, imm, taken);
		}
		Instruction::Bltu { rs1, rs2, imm } => {
			let taken = read_reg(cpu, rs1) < read_reg(cpu, rs2);
			branch(cpu, imm, taken);
		}
		Instruction::Bgeu { rs1, rs2, imm } => {
			let taken = read_reg(cpu, rs1) >= read_reg(cpu, rs2);
			branch(cpu, imm, taken);
		}
		Instruction::Lui { rd, imm } => {
			write_reg(cpu, rd, imm as u64);
			advance_pc(cpu);
		}
		Instruction::Auipc { rd, imm } => {
			let value = add_signed(cpu.pc, imm);
			write_reg(cpu, rd, value);
			advance_pc(cpu);
		}
		Instruction::Jal { rd, imm } => {
			let link = cpu.pc.wrapping_add(4);
			let target = add_signed(cpu.pc, imm);
			write_reg(cpu, rd, link);
			cpu.pc = target;
		}
	}

	Ok(())
}

fn execute_csr(inst: CsrInstruction, cpu: &mut Cpu) -> Result<(), CpuError> {
	match inst {
		CsrInstruction::Csrrw { rd, rs1_or_uimm, csr } => {
			let rs1 = read_reg(cpu, rs1_or_uimm);
			let prior = read_csr(cpu, csr)?;
			write_csr(cpu, csr, rs1)?;
			write_reg(cpu, rd, prior);
			advance_pc(cpu);
		}
		CsrInstruction::Csrrs { rd, rs1_or_uimm, csr } => {
			let rs1 = read_reg(cpu, rs1_or_uimm);
			let prior = read_csr(cpu, csr)?;
			if rs1 != 0 {
				write_csr(cpu, csr, prior | rs1)?;
			}
			write_reg(cpu, rd, prior);
			advance_pc(cpu);
		}
		CsrInstruction::Csrrc { rd, rs1_or_uimm, csr } => {
			let rs1 = read_reg(cpu, rs1_or_uimm);
			let prior = read_csr(cpu, csr)?;
			if rs1 != 0 {
				write_csr(cpu, csr, prior & !rs1)?;
			}
			write_reg(cpu, rd, prior);
			advance_pc(cpu);
		}
		CsrInstruction::Csrrwi { rd, rs1_or_uimm, csr } => {
			let zimm = rs1_or_uimm as u64;
			let prior = read_csr(cpu, csr)?;
			write_csr(cpu, csr, zimm)?;
			write_reg(cpu, rd, prior);
			advance_pc(cpu);
		}
		CsrInstruction::Csrrsi { rd, rs1_or_uimm, csr } => {
			let zimm = rs1_or_uimm as u64;
			let prior = read_csr(cpu, csr)?;
			if zimm != 0 {
				write_csr(cpu, csr, prior | zimm)?;
			}
			write_reg(cpu, rd, prior);
			advance_pc(cpu);
		}
		CsrInstruction::Csrrci { rd, rs1_or_uimm, csr } => {
			let zimm = rs1_or_uimm as u64;
			let prior = read_csr(cpu, csr)?;
			if zimm != 0 {
				write_csr(cpu, csr, prior & !zimm)?;
			}
			write_reg(cpu, rd, prior);
			advance_pc(cpu);
		}
	}

	Ok(())
}

fn read_reg(cpu: &Cpu, index: u8) -> u64 {
	cpu.read_reg(index)
}

fn write_reg(cpu: &mut Cpu, index: u8, value: u64) {
	cpu.write_reg(index, value);
}

fn read_csr(cpu: &Cpu, csr: u16) -> Result<u64, CpuError> {
	cpu.read_csr(csr)
}

fn write_csr(cpu: &mut Cpu, csr: u16, value: u64) -> Result<(), CpuError> {
	cpu.write_csr(csr, value)
}

fn advance_pc(cpu: &mut Cpu) {
	cpu.pc = cpu.pc.wrapping_add(4);
}

fn branch(cpu: &mut Cpu, imm: i64, taken: bool) {
	if taken {
		cpu.pc = add_signed(cpu.pc, imm);
	} else {
		advance_pc(cpu);
	}
}

fn add_signed(base: u64, offset: i64) -> u64 {
	(base as i64).wrapping_add(offset) as u64
}

fn sign_extend(value: u64, bits: u8) -> u64 {
	let shift = 64u32 - bits as u32;
	((value << shift) as i64 >> shift) as u64
}
