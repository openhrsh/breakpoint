/// Argshape stores function arguments as shapes instead of individual values
/// for smaller storage
// use smallvec::SmallVec;
#[derive(Debug, Clone, PartialEq)]
pub enum ArgShape {
    Undefined,
    Null,
    Bool(bool),
    Number,
    Str,
    Object(Vec<String>),
    Array(usize),
    Function,
    Unknown,
}

impl ArgShape {
    /// How many bytes when serialized
    pub fn serialized_size(&self) -> usize {
        match self {
            Self::Undefined | Self::Null | Self::Number |
            Self::Str | Self::Function | Self::Unknown => 1,
            Self::Bool(_) => 2,
            Self::Array(_) => 5,
            Self::Object(keys) => { 1 + 2 + keys.iter().map(|k| k.len() + 1).sum::<usize>() }
        }
    }
}