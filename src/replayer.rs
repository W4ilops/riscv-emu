use std::cmp::Ordering;
use std::path::Path;

use crate::error::{CpuError, CpuResult};
use crate::snapshot::CpuSnapshot;
use crate::trace::TraceFile;

const REG_NAMES: [&str; 32] = [
    "x0/zero",
    "x1/ra",
    "x2/sp",
    "x3/gp",
    "x4/tp",
    "x5/t0",
    "x6/t1",
    "x7/t2",
    "x8/s0",
    "x9/s1",
    "x10/a0",
    "x11/a1",
    "x12/a2",
    "x13/a3",
    "x14/a4",
    "x15/a5",
    "x16/a6",
    "x17/a7",
    "x18/s2",
    "x19/s3",
    "x20/s4",
    "x21/s5",
    "x22/s6",
    "x23/s7",
    "x24/s8",
    "x25/s9",
    "x26/s10",
    "x27/s11",
    "x28/t3",
    "x29/t4",
    "x30/t5",
    "x31/t6",
];

pub struct TraceReplayer {
    trace: TraceFile,
    cursor: u64,
}

impl TraceReplayer {
    pub fn load(path: &Path) -> CpuResult<Self> {
        let trace = TraceFile::read(path)?;
        if trace.snapshots.is_empty() {
            return Err(CpuError::Serialization("trace contains no snapshots".to_string()));
        }
        if trace.header.cycle_count != trace.snapshots.len() as u64 {
            return Err(CpuError::Serialization(
                "trace header cycle count does not match snapshot count".to_string(),
            ));
        }

        Ok(Self { trace, cursor: 0 })
    }

    pub fn goto(&mut self, cycle: u64) -> CpuResult<&CpuSnapshot> {
        let index = self.index_for_cycle(cycle)?;
        self.cursor = index as u64;
        Ok(self.current())
    }

    pub fn step_forward(&mut self) -> CpuResult<&CpuSnapshot> {
        let next = self.cursor + 1;
        if next >= self.trace.snapshots.len() as u64 {
            return Err(CpuError::Serialization("trace cursor at end".to_string()));
        }
        self.cursor = next;
        Ok(self.current())
    }

    pub fn step_back(&mut self) -> CpuResult<&CpuSnapshot> {
        if self.cursor == 0 {
            return Err(CpuError::Serialization("trace cursor at beginning".to_string()));
        }
        self.cursor -= 1;
        Ok(self.current())
    }

    pub fn current(&self) -> &CpuSnapshot {
        &self.trace.snapshots[self.cursor as usize]
    }

    #[allow(dead_code)]
    pub fn total_cycles(&self) -> u64 {
        self.trace.header.cycle_count
    }

    pub fn print_state(&self) {
        let snapshot = self.current();
        let mut lines = Vec::new();
        lines.push(format!(
            "cycle {} pc 0x{:016x}",
            snapshot.cycle, snapshot.pc
        ));

        let mut line = String::new();
        for (idx, value) in snapshot.registers.iter().enumerate() {
            if idx % 4 == 0 && !line.is_empty() {
                lines.push(line);
                line = String::new();
            }
            if !line.is_empty() {
                line.push_str("  ");
            }
            let name = REG_NAMES[idx];
            line.push_str(&format!("{:>6} 0x{:016x}", name, value));
        }
        if !line.is_empty() {
            lines.push(line);
        }

        tracing::info!("{}", lines.join("\n"));
    }

    fn index_for_cycle(&self, cycle: u64) -> CpuResult<usize> {
        let snapshots = &self.trace.snapshots;
        if snapshots.is_empty() {
            return Err(CpuError::Serialization("trace contains no snapshots".to_string()));
        }

        let mut low = 0usize;
        let mut high = snapshots.len();
        while low < high {
            let mid = low + (high - low) / 2;
            match snapshots[mid].cycle.cmp(&cycle) {
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
                Ordering::Equal => return Ok(mid),
            }
        }

        Err(CpuError::Serialization(format!(
            "cycle {} not in trace",
            cycle
        )))
    }
}
