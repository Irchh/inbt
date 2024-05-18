use crate::NbtParseError;
use crate::NbtParseError::{NoSuchValue, TriedGettingFromNonCompound, WrongType};

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
    pub fn type_name(&self) -> String {
        match self {
            NbtTag::End => "end",
            NbtTag::Byte(_, _) => "byte",
            NbtTag::Short(_, _) => "short",
            NbtTag::Int(_, _) => "int",
            NbtTag::Long(_, _) => "long",
            NbtTag::Float(_, _) => "float",
            NbtTag::Double(_, _) => "double",
            NbtTag::ByteArray(_, _) => "byte array",
            NbtTag::String(_, _) => "string",
            NbtTag::List(_, _) => "list",
            NbtTag::Compound(_, _) => "compound",
            NbtTag::IntArray(_, _) => "int array",
            NbtTag::LongArray(_, _) => "long array",
        }.to_string()
    }

    pub fn get<S: Into<String> + Clone>(&self, name: S) -> Result<NbtTag, NbtParseError> {
        match self {
            NbtTag::Compound(_, tags) => {
                for tag in tags {
                    if tag.get_name() == name.clone().into() {
                        return Ok(tag.clone());
                    }
                }
                Err(NoSuchValue(name.into()))
            }
            _ => Err(TriedGettingFromNonCompound(self.type_name()))
        }
    }

    pub fn get_byte<S: Into<String> + Clone>(&self, name: S) -> Result<i8, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::Byte(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("byte".to_string(), tag.type_name()))
        }
    }

    pub fn get_short<S: Into<String> + Clone>(&self, name: S) -> Result<i16, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::Short(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("short".to_string(), tag.type_name()))
        }
    }

    pub fn get_int<S: Into<String> + Clone>(&self, name: S) -> Result<i32, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::Int(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("int".to_string(), tag.type_name()))
        }
    }

    pub fn get_long<S: Into<String> + Clone>(&self, name: S) -> Result<i64, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::Long(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("long".to_string(), tag.type_name()))
        }
    }

    pub fn get_float<S: Into<String> + Clone>(&self, name: S) -> Result<f32, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::Float(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("float".to_string(), tag.type_name()))
        }
    }

    pub fn get_double<S: Into<String> + Clone>(&self, name: S) -> Result<f64, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::Double(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("double".to_string(), tag.type_name()))
        }
    }

    pub fn get_byte_array<S: Into<String> + Clone>(&self, name: S) -> Result<Vec<i8>, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::ByteArray(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("byte array".to_string(), tag.type_name()))
        }
    }

    pub fn get_string<S: Into<String> + Clone>(&self, name: S) -> Result<String, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::String(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("string".to_string(), tag.type_name()))
        }
    }

    pub fn get_list<S: Into<String> + Clone>(&self, name: S) -> Result<Vec<NbtTag>, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::List(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("list".to_string(), tag.type_name()))
        }
    }

    pub fn get_compound<S: Into<String> + Clone>(&self, name: S) -> Result<Vec<NbtTag>, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::Compound(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("compount".to_string(), tag.type_name()))
        }
    }

    pub fn get_int_array<S: Into<String> + Clone>(&self, name: S) -> Result<Vec<i32>, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::IntArray(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("int array".to_string(), tag.type_name()))
        }
    }

    pub fn get_long_array<S: Into<String> + Clone>(&self, name: S) -> Result<Vec<i64>, NbtParseError> {
        let tag = self.get(name)?;
        if let NbtTag::LongArray(_, value) = tag {
            Ok(value)
        } else {
            Err(WrongType("long array".to_string(), tag.type_name()))
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