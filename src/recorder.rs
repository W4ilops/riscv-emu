use std::path::Path;

use crate::cpu::Cpu;
use crate::error::CpuError;
use crate::snapshot::CpuSnapshot;
use crate::trace::TraceFile;

pub struct SnapshotRecorder {
    cpu: Cpu,
    snapshots: Vec<CpuSnapshot>,
    cycle: u64,
}

#[allow(dead_code)]
impl SnapshotRecorder {
    pub fn new(cpu: Cpu) -> Self {
        let mut recorder = Self {
            cpu,
            snapshots: Vec::new(),
            cycle: 0,
        };
        recorder.snapshots.push(recorder.capture_snapshot());
        recorder
    }

    pub fn step(&mut self) -> Result<(), CpuError> {
        self.cpu.step()?;
        self.cycle = self.cycle.wrapping_add(1);
        self.snapshots.push(self.capture_snapshot());
        Ok(())
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

    pub fn save(&self, path: &Path) -> Result<(), CpuError> {
        TraceFile::write(&self.snapshots, path)
    }

    pub fn snapshot_at(&self, cycle: u64) -> Option<&CpuSnapshot> {
        self.snapshots.iter().find(|snapshot| snapshot.cycle == cycle)
    }

    pub fn snapshots(&self) -> &[CpuSnapshot] {
        &self.snapshots
    }

    pub fn cycle_count(&self) -> u64 {
        self.cycle
    }

    fn capture_snapshot(&self) -> CpuSnapshot {
        let mut csrs: Vec<(u16, u64)> = self.cpu.csrs.iter().map(|(addr, value)| (*addr, *value)).collect();
        csrs.sort_by_key(|(addr, _)| *addr);

        CpuSnapshot {
            cycle: self.cycle,
            pc: self.cpu.pc,
            registers: self.cpu.registers,
            csrs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SnapshotRecorder;
    use crate::bus::Bus;
    use crate::cpu::Cpu;
    use crate::trace::TraceFile;

    const RAM_BASE: u64 = 0x8000_0000;
    const EBREAK: u32 = 0x0010_0073;

    fn encode_i(imm: i32, rs1: u8, funct3: u32, rd: u8, opcode: u32) -> u32 {
        let imm = (imm as u32) & 0x0fff;
        (imm << 20)
            | ((rs1 as u32) << 15)
            | (funct3 << 12)
            | ((rd as u32) << 7)
            | opcode
    }

    fn load_recorder(program: &[u32]) -> SnapshotRecorder {
        let mut bus = Bus::new(0x2000);
        let mut bytes = Vec::with_capacity(program.len() * 4);
        for word in program {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        bus.load_bytes(RAM_BASE, &bytes).unwrap();
        let cpu = Cpu::new(RAM_BASE, bus);
        SnapshotRecorder::new(cpu)
    }

    fn build_program() -> Vec<u32> {
        let mut program = Vec::new();
        program.push(encode_i(1, 0, 0b000, 1, 0b001_0011));
        for _ in 0..5 {
            program.push(encode_i(1, 1, 0b000, 1, 0b001_0011));
        }
        program.push(EBREAK);
        program
    }

    #[test]
    fn test_record_and_seek() {
        let program = build_program();
        let mut recorder = load_recorder(&program);
        recorder.run().unwrap();

        let entry = RAM_BASE;
        assert_eq!(recorder.snapshot_at(0).unwrap().pc, entry);
        assert_eq!(recorder.snapshot_at(3).unwrap().registers[1], 3);
        assert_eq!(recorder.cycle_count(), 6);
    }

    #[test]
    fn test_trace_roundtrip() {
        let program = build_program();
        let mut recorder = load_recorder(&program);
        recorder.run().unwrap();

        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("riscv-emu-trace-{}.trace", nanos));

        TraceFile::write(recorder.snapshots(), &path).unwrap();
        let trace = TraceFile::read(&path).unwrap();

        assert_eq!(trace.snapshots.len(), recorder.snapshots().len());
        let index = 3usize;
        assert_eq!(
            trace.snapshots[index].registers,
            recorder.snapshots()[index].registers
        );

        let _ = std::fs::remove_file(&path);
    }
}
