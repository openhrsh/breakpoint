//the trace file manager, high speed write and read
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, BufReader, Write, Read, Seek, SeekFrom};
use std::path::Path;