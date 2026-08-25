#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventKind { //MUST integer map
    FnCall = 0,
    FnReturn = 1,
    ConsoleLog = 2,
    DomMutation = 3,
    Error = 4,
    MemWrite = 5,
    RegState = 6,
    //TODO: add more event kinds
}

impl EventKind {
    pub fn from_u8(byte: u8) -> Option<Self> { //deserializing
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
    pub ts: u64,
    pub kind: EventKind,
}

impl TraceEvent {
}