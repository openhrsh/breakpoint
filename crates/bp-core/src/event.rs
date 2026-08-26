#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] //tells rust to store EventKind as a u8
pub enum EventKind { //MUST integer map to avoid compiler sequential assigning when inserting new events in between
    FnCall = 0,
    FnReturn = 1,
    ConsoleLog = 2,
    DomMutation = 3,
    Error = 4,
    MemWrite = 5,
    RegState = 6,
    //TODO: add more event kinds, I think 256 exist
}

impl EventKind {
    pub fn from_u8(byte: u8) -> Option<Self> { //convert the stored u8 EventKind to enum 
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