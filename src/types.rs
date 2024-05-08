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