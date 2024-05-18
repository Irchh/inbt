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
}