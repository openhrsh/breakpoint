//the trace file manager, high speed write and read
use std::fs::{File, OpenOptions, read};
use std::io::{BufWriter, BufReader, Write, Read, Seek, SeekFrom};
use std::path::Path;
use crate::event::TraceEvent;
use crate::intern::InternTable;

/// Struct only for writing concerns
pub struct BinaryLog {
    writer: BufWriter<File>,
    event_count: u64,
}

impl BinaryLog {
    /// Create binary trace file
    pub fn create(path: &Path, intern_table: &InternTable) -> std::io::Result<Self> {
        let file = File::create(path)?;
        let mut writer = BufWriter::with_capacity(64 * 1024, file);

        let table_bytes = intern_table.to_bytes();
        writer.write_all(&(table_bytes.len() as u32).to_le_bytes())?;
        writer.write_all(&table_bytes)?;

        Ok(Self { writer, event_count:0 })
    }

    //NOTE: make sure flush is called before dropping BufWriter
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }

    pub fn event_count(&self) -> u64 {
        self.event_count
    }

    // TODO: finish TraceEvent impl
    // pub fn write_event(&mut self, event: &TraceEvent) {}
}

/// Struct only for reading concerns
pub struct TraceReader {
    reader: BufReader<File>,
    pub intern_table: InternTable,
    pub event_count: u64,
}

impl TraceReader {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;
        let table_len = u32::from_le_bytes(len_buf) as usize;

        let mut table_bytes = vec![0u8; table_len];
        reader.read_exact(&mut table_bytes)?;
        let intern_table = InternTable::from_bytes(&table_bytes).
            ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "corrupt intern table"))?;

        Ok(Self { reader, intern_table, event_count:0 })
    }
}