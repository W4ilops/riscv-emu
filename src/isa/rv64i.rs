#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
	// R-type
	Add { rd: u8, rs1: u8, rs2: u8 },
	Sub { rd: u8, rs1: u8, rs2: u8 },
	Sll { rd: u8, rs1: u8, rs2: u8 },
	Slt { rd: u8, rs1: u8, rs2: u8 },
	Sltu { rd: u8, rs1: u8, rs2: u8 },
	Xor { rd: u8, rs1: u8, rs2: u8 },
	Srl { rd: u8, rs1: u8, rs2: u8 },
	Sra { rd: u8, rs1: u8, rs2: u8 },
	Or { rd: u8, rs1: u8, rs2: u8 },
	And { rd: u8, rs1: u8, rs2: u8 },
	Addw { rd: u8, rs1: u8, rs2: u8 },
	Subw { rd: u8, rs1: u8, rs2: u8 },
	Sllw { rd: u8, rs1: u8, rs2: u8 },
	Srlw { rd: u8, rs1: u8, rs2: u8 },
	Sraw { rd: u8, rs1: u8, rs2: u8 },

	// I-type
	Addi { rd: u8, rs1: u8, imm: i64 },
	Slti { rd: u8, rs1: u8, imm: i64 },
	Sltiu { rd: u8, rs1: u8, imm: i64 },
	Xori { rd: u8, rs1: u8, imm: i64 },
	Ori { rd: u8, rs1: u8, imm: i64 },
	Andi { rd: u8, rs1: u8, imm: i64 },
	Slli { rd: u8, rs1: u8, shamt: u8 },
	Srli { rd: u8, rs1: u8, shamt: u8 },
	Srai { rd: u8, rs1: u8, shamt: u8 },
	Addiw { rd: u8, rs1: u8, imm: i64 },
	Slliw { rd: u8, rs1: u8, shamt: u8 },
	Srliw { rd: u8, rs1: u8, shamt: u8 },
	Sraiw { rd: u8, rs1: u8, shamt: u8 },
	Lb { rd: u8, rs1: u8, imm: i64 },
	Lh { rd: u8, rs1: u8, imm: i64 },
	Lw { rd: u8, rs1: u8, imm: i64 },
	Ld { rd: u8, rs1: u8, imm: i64 },
	Lbu { rd: u8, rs1: u8, imm: i64 },
	Lhu { rd: u8, rs1: u8, imm: i64 },
	Lwu { rd: u8, rs1: u8, imm: i64 },
	Jalr { rd: u8, rs1: u8, imm: i64 },
	Fence { pred: u8, succ: u8 },
	FenceI,
	Ecall,
	Ebreak,

	// S-type
	Sb { rs1: u8, rs2: u8, imm: i64 },
	Sh { rs1: u8, rs2: u8, imm: i64 },
	Sw { rs1: u8, rs2: u8, imm: i64 },
	Sd { rs1: u8, rs2: u8, imm: i64 },

	// B-type
	Beq { rs1: u8, rs2: u8, imm: i64 },
	Bne { rs1: u8, rs2: u8, imm: i64 },
	Blt { rs1: u8, rs2: u8, imm: i64 },
	Bge { rs1: u8, rs2: u8, imm: i64 },
	Bltu { rs1: u8, rs2: u8, imm: i64 },
	Bgeu { rs1: u8, rs2: u8, imm: i64 },

	// U-type
	Lui { rd: u8, imm: i64 },
	Auipc { rd: u8, imm: i64 },

	// J-type
	Jal { rd: u8, imm: i64 },
}
