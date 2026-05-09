pub mod rv64i;
pub mod zicsr;

pub use rv64i::Instruction;
pub use zicsr::CsrInstruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnyInstruction {
	Base(Instruction),
	Csr(CsrInstruction),
}
