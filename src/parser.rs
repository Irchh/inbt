#![allow(dead_code)]
use std::iter::Peekable;
use std::slice::Iter;
use crate::parse_error::ParseError;
use crate::types::NbtTag;

pub struct NbtParser;

impl NbtParser {
    pub fn parse_binary(data_vec: Vec<u8>) -> NbtTag {
        let mut tree = NbtTag::Compound("".to_string(), vec![]);
        let mut data = data_vec.iter().peekable();

        let mut is_first = true;

        while let Some(id) = data.next() {
            let node = match *id {
                1 => Self::parse_byte(&mut data),
                2 => Self::parse_short(&mut data),
                3 => Self::parse_int(&mut data),
                4 => Self::parse_long(&mut data),
                _ => panic!("Unknown NBT ID: {}", id),
            };

            if node.is_err() {
                panic!("Error: {}", node.err().unwrap())
            }

            if is_first {
                is_first = false;
                tree = node.unwrap();
            } else {
                todo!()
            }
        }

        tree
    }

    fn parse_end(_: &mut Peekable<Iter<u8>>) -> NbtTag {
        NbtTag::End
    }

    fn next(iterable: &mut Peekable<Iter<u8>>) -> Result<u8, ParseError>{
        iterable.next().map(|n| *n).ok_or(ParseError::EndOfData)
    }

    fn next_byte(iterable: &mut Peekable<Iter<u8>>) -> Result<i8, ParseError> {
        Ok(i8::from_be_bytes([Self::next(iterable)?]))
    }

    fn next_short(iterable: &mut Peekable<Iter<u8>>) -> Result<i16, ParseError> {
        Ok(i16::from_be_bytes([Self::next(iterable)?, Self::next(iterable)?]))
    }

    fn next_int(iterable: &mut Peekable<Iter<u8>>) -> Result<i32, ParseError> {
        Ok(i32::from_be_bytes([
            Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?
        ]))
    }

    fn next_long(iterable: &mut Peekable<Iter<u8>>) -> Result<i64, ParseError> {
        Ok(i64::from_be_bytes([
            Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?,
            Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?
        ]))
    }

    pub(super) fn parse_name(iterable: &mut Peekable<Iter<u8>>) -> Result<String, ParseError> {
        let size = Self::next_short(iterable)?;
        let name = String::from_utf8(iterable.take(size as usize).map(|n| *n).collect::<Vec<u8>>())?;
        if name.as_bytes().len() < size as usize {
            return Err(ParseError::EndOfData);
        }
        Ok(name)
    }

    pub(super) fn parse_byte(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::parse_name(iterable)?;
        Ok(NbtTag::Byte(name, Self::next_byte(iterable)?))
    }

    pub(super) fn parse_short(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError>{
        let name = Self::parse_name(iterable)?;
        Ok(NbtTag::Short(name, Self::next_short(iterable)?))
    }

    pub(super) fn parse_int(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError>{
        let name = Self::parse_name(iterable)?;
        Ok(NbtTag::Int(name, Self::next_int(iterable)?))
    }

    pub(super) fn parse_long(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError>{
        let name = Self::parse_name(iterable)?;
        Ok(NbtTag::Long(name, Self::next_long(iterable)?))
    }
}