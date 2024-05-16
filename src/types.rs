/// Enum type for all existing nbt types. Excluding End, each tag contains a name as a String
/// and it's value.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum NbtTag {
    /// Denotes the end of List or Compound
    End,
    /// 8-bit signed integer
    Byte(String, i8),
    /// 16-bit signed integer
    Short(String, i16),
    /// 32-bit signed integer
    Int(String, i32),
    /// 64-bit signed integer
    Long(String, i64),
    /// 32-bit IEEE 754-2008 float
    Float(String, f32),
    /// 64-bit IEEE 754-2008 float
    Double(String, f64),
    /// Vector of 8-bit signed integers
    ByteArray(String, Vec<i8>),
    /// A normal string
    String(String, String),
    /// Vector of NbtTag, all elements are the same type
    List(String, Vec<NbtTag>),
    /// Vector of NbtTag, all elements can be different types
    Compound(String, Vec<NbtTag>),
    /// Vector of 32-bit signed integers
    IntArray(String, Vec<i32>),
    /// Vector of 64-bit signed integers
    LongArray(String, Vec<i64>),
}

impl NbtTag {
    pub fn get<S: Into<String> + Clone>(&self, name: S) -> Option<NbtTag> {
        match self {
            NbtTag::Compound(_, tags) => {
                for tag in tags {
                    if tag.get_name() == name.clone().into() {
                        return Some(tag.clone());
                    }
                }
                None
            }
            _ => None
        }
    }

    pub fn get_int<S: Into<String> + Clone>(&self, name: S) -> Option<i32> {
        if let NbtTag::Int(_, value) = self.get(name)? {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            NbtTag::End => "".to_string(),
            NbtTag::Byte(name, _) => name.clone(),
            NbtTag::Short(name, _) => name.clone(),
            NbtTag::Int(name, _) => name.clone(),
            NbtTag::Long(name, _) => name.clone(),
            NbtTag::Float(name, _) => name.clone(),
            NbtTag::Double(name, _) => name.clone(),
            NbtTag::ByteArray(name, _) => name.clone(),
            NbtTag::String(name, _) => name.clone(),
            NbtTag::List(name, _) => name.clone(),
            NbtTag::Compound(name, _) => name.clone(),
            NbtTag::IntArray(name, _) => name.clone(),
            NbtTag::LongArray(name, _) => name.clone(),
        }
    }
}