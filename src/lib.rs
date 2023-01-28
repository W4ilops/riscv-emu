// Library interface for benchmarking and testing

pub mod bus;
pub mod cpu;
pub mod decode;
pub mod error;
pub mod execute;
pub mod isa;
pub mod memory;
pub mod recorder;
pub mod replayer;
pub mod snapshot;
pub mod trace;

// Re-export commonly used types
pub use bus::Bus;
pub use cpu::Cpu;
pub use error::CpuError;
pub use recorder::SnapshotRecorder;
pub use replayer::TraceReplayer;
pub use snapshot::CpuSnapshot;
