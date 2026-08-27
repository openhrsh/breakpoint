use std::collections::HashMap;

/// A dictionary that converts strings to small ints
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternTable {
    forward: HashMap<String, u32>, //intern new strings
    reverse: Vec<String>, //resolve ids' to strings
}

impl InternTable {
    pub fn new() -> Self {
        Self {
            forward: HashMap::with_capacity(1024),
            reverse: Vec::with_capacity(1024),
        }
    }

    pub fn intern(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.forward.get(name) {
            return id; //fast O(1) lookup
        }

        let id = self.reverse.len() as u32;
        self.reverse.push(name.to_owned());
        self.forward.insert(name.to_owned(), id);
        id
    }

    /// Resolve id back to string, O(1) vec index.
    /// Returns borrowed &str, no allocation
    pub fn resolve(&self, id: u32) -> Option<&str> {
        self.reverse.get(id as usize).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.reverse.len()
    }

    pub fn is_empty(&self) -> bool {
        self.reverse.is_empty()
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&(self.reverse.len() as u32).to_le_bytes());
        for name in &self.reverse {
            //write each string as: u16 length + utf8 bytes
            buf.extend_from_slice(&(name.len() as u16).to_le_bytes());
            buf.extend_from_slice(name.as_bytes());
        }
        buf
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut table = Self::new();
        let mut pos = 0;
        let count = u32::from_le_bytes(bytes[pos..pos+4].try_into().ok()?) as usize;
        pos += 4;
        for _ in 0..count {
            let len = u16::from_le_bytes(bytes[pos..pos+2].try_into().ok()?) as usize;
            pos += 2;
            let name = std::str::from_utf8(&bytes[pos..pos+len]).ok()?;
            table.intern(name);
            pos += len;
        }
        Some(table)
    }
}

impl Default for InternTable {
    fn default() -> Self {
        Self::new()
    }
}