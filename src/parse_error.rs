use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NbtParseError {
    #[error("Failed parsing UTF-8 string: {0}")]
    StringUtf8Error(#[from] FromUtf8Error),
    #[error("Hit end of data")]
    EndOfData,
    #[error("Unknown NBT type: {0}")]
    UnknownNBT(i8),
    #[error("No such value: {0}")]
    NoSuchValue(String),
    #[error("Expected type {0}, got {1}")]
    WrongType(String, String),
    #[error("Tried to get named child of non-compound tag. Actual type is {0}")]
    TriedGettingFromNonCompound(String),
}