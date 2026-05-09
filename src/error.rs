use thiserror::Error;

#[derive(Debug, Error)]
pub enum CpuError {
    #[error("illegal instruction 0x{0:08x}")]
    IllegalInstruction(u32),
    #[error("misaligned access addr=0x{addr:x} width={width}")]
    Misaligned { addr: u64, width: u8 },
    #[error("bus error addr=0x{0:x}")]
    BusError(u64),
    #[allow(dead_code)]
    #[error("unknown csr 0x{0:04x}")]
    UnknownCsr(u16),
    #[error("ebreak")]
    Ebreak,
    #[error("ecall")]
    Ecall,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serialization(String),
}

#[allow(dead_code)]
pub type CpuResult<T> = Result<T, CpuError>;
