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

        loop {
            if data.len() == 0 {
                break;
            }

            let node = Self::parse_next(&mut data);

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

    fn parse_next(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        match Self::next_byte(iterable)? {
            0 => Self::parse_end(iterable),
            1 => Self::parse_byte(iterable),
            2 => Self::parse_short(iterable),
            3 => Self::parse_int(iterable),
            4 => Self::parse_long(iterable),
            5 => Self::parse_float(iterable),
            6 => Self::parse_double(iterable),
            7 => Self::parse_byte_arr(iterable),
            8 => Self::parse_string(iterable),
            9 => Self::parse_list(iterable),
            10 => Self::parse_compound(iterable),
            11 => Self::parse_int_array(iterable),
            12 => Self::parse_long_array(iterable),
            id => Err(ParseError::UnknownNBT(id)),
        }
    }

    fn parse_end(_: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        Ok(NbtTag::End)
    }

    fn next(iterable: &mut Peekable<Iter<u8>>) -> Result<u8, ParseError> {
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

    fn next_float(iterable: &mut Peekable<Iter<u8>>) -> Result<f32, ParseError> {
        Ok(f32::from_be_bytes([
            Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?
        ]))
    }

    fn next_double(iterable: &mut Peekable<Iter<u8>>) -> Result<f64, ParseError> {
        Ok(f64::from_be_bytes([
            Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?,
            Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?, Self::next(iterable)?
        ]))
    }

    fn next_byte_arr(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<i8>, ParseError> {
        let size = Self::next_int(iterable)?;
        let arr = iterable.take(size as usize).map(|n| *n as i8).collect::<Vec<i8>>();
        if arr.len() < size as usize {
            return Err(ParseError::EndOfData);
        }
        Ok(arr)
    }

    fn next_string(iterable: &mut Peekable<Iter<u8>>) -> Result<String, ParseError> {
        let size = Self::next_short(iterable)?;
        let name = String::from_utf8(iterable.take(size as usize).map(|n| *n).collect::<Vec<u8>>())?;
        if name.as_bytes().len() < size as usize {
            return Err(ParseError::EndOfData);
        }
        Ok(name)
    }

    fn next_list(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<NbtTag>, ParseError> {
        let tag_id = Self::next_byte(iterable)?;
        let size = Self::next_int(iterable)?;
        let mut vec = vec![];
        for _ in 0..size {
            let value = match tag_id {
                1 => NbtTag::Byte("".to_string(), Self::next_byte(iterable)?),
                2 => NbtTag::Short("".to_string(), Self::next_short(iterable)?),
                3 => NbtTag::Int("".to_string(), Self::next_int(iterable)?),
                4 => NbtTag::Long("".to_string(), Self::next_long(iterable)?),
                5 => NbtTag::Float("".to_string(), Self::next_float(iterable)?),
                6 => NbtTag::Double("".to_string(), Self::next_double(iterable)?),
                7 => NbtTag::ByteArray("".to_string(), Self::next_byte_arr(iterable)?),
                8 => NbtTag::String("".to_string(), Self::next_string(iterable)?),
                9 => NbtTag::List("".to_string(), Self::next_list(iterable)?),
                10 => NbtTag::List("".to_string(), Self::next_compound(iterable)?),
                11 => NbtTag::IntArray("".to_string(), Self::next_int_arr(iterable)?),
                12 => NbtTag::LongArray("".to_string(), Self::next_long_arr(iterable)?),
                _ => Err(ParseError::UnknownNBT(tag_id))?,
            };
            vec.push(value);
        }
        Ok(vec)
    }

    fn next_compound(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<NbtTag>, ParseError> {
        let mut vec = vec![];
        loop {
            let next = Self::parse_next(iterable)?;
            if next == NbtTag::End {
                break;
            }
            vec.push(next);
        }
        Ok(vec)
    }

    fn next_int_arr(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<i32>, ParseError> {
        let size = Self::next_int(iterable)?;
        let mut vec = vec![];
        for _ in 0..size {
            vec.push(Self::next_int(iterable)?)
        }
        Ok(vec)
    }

    fn next_long_arr(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<i64>, ParseError> {
        let size = Self::next_int(iterable)?;
        let mut vec = vec![];
        for _ in 0..size {
            vec.push(Self::next_long(iterable)?)
        }
        Ok(vec)
    }

    pub(super) fn parse_byte(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        Ok(NbtTag::Byte(name, Self::next_byte(iterable)?))
    }

    pub(super) fn parse_short(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        Ok(NbtTag::Short(name, Self::next_short(iterable)?))
    }

    pub(super) fn parse_int(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        Ok(NbtTag::Int(name, Self::next_int(iterable)?))
    }

    pub(super) fn parse_long(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        Ok(NbtTag::Long(name, Self::next_long(iterable)?))
    }

    pub(super) fn parse_float(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        Ok(NbtTag::Float(name, Self::next_float(iterable)?))
    }

    pub(super) fn parse_double(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        Ok(NbtTag::Double(name, Self::next_double(iterable)?))
    }

    pub(super) fn parse_byte_arr(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        Ok(NbtTag::ByteArray(name, Self::next_byte_arr(iterable)?))
    }

    pub(super) fn parse_string(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        let string = Self::next_string(iterable)?;
        Ok(NbtTag::String(name, string))
    }

    pub(super) fn parse_list(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        let vec = Self::next_list(iterable)?;
        Ok(NbtTag::List(name, vec))
    }

    pub(super) fn parse_compound(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        let vec = Self::next_compound(iterable)?;
        Ok(NbtTag::Compound(name, vec))
    }

    pub(super) fn parse_int_array(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        let vec = Self::next_int_arr(iterable)?;
        Ok(NbtTag::IntArray(name, vec))
    }

    pub(super) fn parse_long_array(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, ParseError> {
        let name = Self::next_string(iterable)?;
        let vec = Self::next_long_arr(iterable)?;
        Ok(NbtTag::LongArray(name, vec))
    }
}