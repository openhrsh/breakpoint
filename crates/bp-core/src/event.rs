/// Represents the type of operation captured in the trace.
/// Use explicit integer mapping (repr[u8]) to prevent the compiler
/// from automatically shifting values if we insert new event types later
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventKind {
    FnCall = 0,
    FnReturn = 1,
    ConsoleLog = 2,
    DomMutation = 3,
    Error = 4,
    MemWrite = 5,
    RegState = 6,
    //TODO: Map remaining 250 event types
}

impl EventKind {
    /// Convert the stored u8 EventKind to enum
    pub fn from_u8(byte: u8) -> Option<Self> { 
        match byte {
            0 => Some(EventKind::FnCall),
            1 => Some(EventKind::FnReturn),
            2 => Some(EventKind::ConsoleLog),
            3 => Some(EventKind::DomMutation),
            4 => Some(EventKind::Error),
            5 => Some(EventKind::MemWrite),
            6 => Some(EventKind::RegState),
            _ => None,
        }
    }
}

#[repr(C)]
pub struct TraceEvent {
    pub ts: u64, //8bytes
    pub kind: u8, //1byte: EventKind as u8
    pub fn_id: u32, //4bytes: interned function ID
    pub flags: u8, //1byte: packed boolean flags
    pub payload_len: u16, //2bytes: args bytes payload
    //total fixed header: 16 bytes
}

impl TraceEvent {

    pub fn elapsed_ms(&self, session_start: u64) -> f64 {
        (self.ts - session_start) as f64 / 1000.0
    }
}