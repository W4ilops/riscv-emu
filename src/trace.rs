use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};

use crate::error::{CpuError, CpuResult};
use crate::snapshot::CpuSnapshot;

const TRACE_MAGIC: [u8; 4] = *b"RVET";
const TRACE_VERSION: u8 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceHeader {
    pub magic: [u8; 4],
    pub version: u8,
    pub cycle_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceFile {
    pub header: TraceHeader,
    pub snapshots: Vec<CpuSnapshot>,
}

impl TraceFile {
    pub fn write(snapshots: &[CpuSnapshot], path: &Path) -> CpuResult<()> {
        let header = TraceHeader {
            magic: TRACE_MAGIC,
            version: TRACE_VERSION,
            cycle_count: snapshots.len() as u64,
        };

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let mut encoder = GzEncoder::new(writer, Compression::default());

        bincode::serialize_into(&mut encoder, &header).map_err(map_bincode_error)?;
        bincode::serialize_into(&mut encoder, snapshots).map_err(map_bincode_error)?;
        encoder.finish().map(|_| ()).map_err(CpuError::Io)
    }

    pub fn read(path: &Path) -> CpuResult<TraceFile> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut decoder = GzDecoder::new(reader);

        let header: TraceHeader = bincode::deserialize_from(&mut decoder).map_err(map_bincode_error)?;
        if header.magic != TRACE_MAGIC {
            return Err(CpuError::Serialization("invalid trace magic".to_string()));
        }

        let snapshots: Vec<CpuSnapshot> =
            bincode::deserialize_from(&mut decoder).map_err(map_bincode_error)?;

        Ok(TraceFile { header, snapshots })
    }
}

#[allow(clippy::boxed_local)]
fn map_bincode_error(err: Box<bincode::ErrorKind>) -> CpuError {
    match *err {
        bincode::ErrorKind::Io(io_err) => CpuError::Io(io_err),
        other => CpuError::Serialization(other.to_string()),
    }
}
