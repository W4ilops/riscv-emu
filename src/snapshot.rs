use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuSnapshot {
    pub cycle: u64,
    pub pc: u64,
    pub registers: [u64; 32],
    pub csrs: Vec<(u16, u64)>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotDiff {
    pub cycle: Option<u64>,
    pub pc: Option<u64>,
    pub registers: Vec<(u8, u64)>,
    pub csrs: Vec<(u16, u64)>,
    pub csrs_removed: Vec<u16>,
}

#[allow(dead_code)]
impl CpuSnapshot {
    pub fn diff(&self, prev: &CpuSnapshot) -> SnapshotDiff {
        let cycle = if self.cycle != prev.cycle {
            Some(self.cycle)
        } else {
            None
        };
        let pc = if self.pc != prev.pc { Some(self.pc) } else { None };

        let mut registers = Vec::new();
        for (idx, value) in self.registers.iter().enumerate() {
            if *value != prev.registers[idx] {
                registers.push((idx as u8, *value));
            }
        }

        let prev_csrs = map_csrs(&prev.csrs);
        let next_csrs = map_csrs(&self.csrs);

        let mut csrs = Vec::new();
        for (addr, value) in next_csrs.iter() {
            match prev_csrs.get(addr) {
                Some(prev_value) if prev_value == value => {}
                _ => csrs.push((*addr, *value)),
            }
        }

        let mut csrs_removed = Vec::new();
        for addr in prev_csrs.keys() {
            if !next_csrs.contains_key(addr) {
                csrs_removed.push(*addr);
            }
        }

        SnapshotDiff {
            cycle,
            pc,
            registers,
            csrs,
            csrs_removed,
        }
    }
}

#[allow(dead_code)]
impl SnapshotDiff {
    pub fn apply_to(&self, prev: &CpuSnapshot) -> CpuSnapshot {
        let cycle = self.cycle.unwrap_or(prev.cycle);
        let pc = self.pc.unwrap_or(prev.pc);

        let mut registers = prev.registers;
        for (idx, value) in &self.registers {
            let reg = *idx as usize;
            if reg < registers.len() {
                registers[reg] = *value;
            }
        }

        let mut csrs = map_csrs(&prev.csrs);
        for addr in &self.csrs_removed {
            csrs.remove(addr);
        }
        for (addr, value) in &self.csrs {
            csrs.insert(*addr, *value);
        }

        CpuSnapshot {
            cycle,
            pc,
            registers,
            csrs: csrs.into_iter().collect(),
        }
    }
}

#[allow(dead_code)]
fn map_csrs(entries: &[(u16, u64)]) -> BTreeMap<u16, u64> {
    let mut map = BTreeMap::new();
    for (addr, value) in entries {
        map.insert(*addr, *value);
    }
    map
}
