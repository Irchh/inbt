#![allow(dead_code)]
use std::iter::Peekable;
use std::slice::Iter;
use crate::parse_error::NbtParseError;
use crate::types::NbtTag;

pub mod nbt_parser {
    use std::io;
    use std::io::Read;
    use flate2::read::{GzDecoder, ZlibDecoder};
    use super::*;

    pub fn parse_gzip(compressed: Vec<u8>) -> io::Result<NbtTag> {
        let mut decompresser = GzDecoder::new(compressed.as_slice());
        let mut data = vec![];
        let _ = decompresser.read_to_end(&mut data)?;
        Ok(parse_binary(data))
    }

    pub fn parse_zlib(compressed: Vec<u8>) -> io::Result<NbtTag> {
        let mut decompresser = ZlibDecoder::new(compressed.as_slice());
        let mut data = vec![];
        let _ = decompresser.read_to_end(&mut data)?;
        Ok(parse_binary(data))
    }

    pub fn parse_binary(data_vec: Vec<u8>) -> NbtTag {
        let mut data = data_vec.iter().peekable();
        parse_next(&mut data).unwrap()
    }

    fn parse_next(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        match next_byte(iterable)? {
            0 => parse_end(iterable),
            1 => parse_byte(iterable),
            2 => parse_short(iterable),
            3 => parse_int(iterable),
            4 => parse_long(iterable),
            5 => parse_float(iterable),
            6 => parse_double(iterable),
            7 => parse_byte_arr(iterable),
            8 => parse_string(iterable),
            9 => parse_list(iterable),
            10 => parse_compound(iterable),
            11 => parse_int_array(iterable),
            12 => parse_long_array(iterable),
            id => Err(NbtParseError::UnknownNBT(id)),
        }
    }

    fn parse_end(_: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        Ok(NbtTag::End)
    }

    fn next(iterable: &mut Peekable<Iter<u8>>) -> Result<u8, NbtParseError> {
        iterable.next().map(|n| *n).ok_or(NbtParseError::EndOfData)
    }

    fn next_byte(iterable: &mut Peekable<Iter<u8>>) -> Result<i8, NbtParseError> {
        Ok(i8::from_be_bytes([next(iterable)?]))
    }

    fn next_short(iterable: &mut Peekable<Iter<u8>>) -> Result<i16, NbtParseError> {
        Ok(i16::from_be_bytes([next(iterable)?, next(iterable)?]))
    }

    fn next_int(iterable: &mut Peekable<Iter<u8>>) -> Result<i32, NbtParseError> {
        Ok(i32::from_be_bytes([
            next(iterable)?, next(iterable)?, next(iterable)?, next(iterable)?
        ]))
    }

    fn next_long(iterable: &mut Peekable<Iter<u8>>) -> Result<i64, NbtParseError> {
        Ok(i64::from_be_bytes([
            next(iterable)?, next(iterable)?, next(iterable)?, next(iterable)?,
            next(iterable)?, next(iterable)?, next(iterable)?, next(iterable)?
        ]))
    }

    fn next_float(iterable: &mut Peekable<Iter<u8>>) -> Result<f32, NbtParseError> {
        Ok(f32::from_be_bytes([
            next(iterable)?, next(iterable)?, next(iterable)?, next(iterable)?
        ]))
    }

    fn next_double(iterable: &mut Peekable<Iter<u8>>) -> Result<f64, NbtParseError> {
        Ok(f64::from_be_bytes([
            next(iterable)?, next(iterable)?, next(iterable)?, next(iterable)?,
            next(iterable)?, next(iterable)?, next(iterable)?, next(iterable)?
        ]))
    }

    fn next_byte_arr(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<i8>, NbtParseError> {
        let size = next_int(iterable)?;
        let arr = iterable.take(size as usize).map(|n| *n as i8).collect::<Vec<i8>>();
        if arr.len() < size as usize {
            return Err(NbtParseError::EndOfData);
        }
        Ok(arr)
    }

    fn next_string(iterable: &mut Peekable<Iter<u8>>) -> Result<String, NbtParseError> {
        let size = next_short(iterable)?;
        let name = String::from_utf8(iterable.take(size as usize).map(|n| *n).collect::<Vec<u8>>())?;
        if name.as_bytes().len() < size as usize {
            return Err(NbtParseError::EndOfData);
        }
        Ok(name)
    }

    fn next_list(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<NbtTag>, NbtParseError> {
        let tag_id = next_byte(iterable)?;
        let size = next_int(iterable)?;
        let mut vec = vec![];
        for _ in 0..size {
            let value = match tag_id {
                1 => NbtTag::Byte("".to_string(), next_byte(iterable)?),
                2 => NbtTag::Short("".to_string(), next_short(iterable)?),
                3 => NbtTag::Int("".to_string(), next_int(iterable)?),
                4 => NbtTag::Long("".to_string(), next_long(iterable)?),
                5 => NbtTag::Float("".to_string(), next_float(iterable)?),
                6 => NbtTag::Double("".to_string(), next_double(iterable)?),
                7 => NbtTag::ByteArray("".to_string(), next_byte_arr(iterable)?),
                8 => NbtTag::String("".to_string(), next_string(iterable)?),
                9 => NbtTag::List("".to_string(), next_list(iterable)?),
                10 => NbtTag::List("".to_string(), next_compound(iterable)?),
                11 => NbtTag::IntArray("".to_string(), next_int_arr(iterable)?),
                12 => NbtTag::LongArray("".to_string(), next_long_arr(iterable)?),
                _ => Err(NbtParseError::UnknownNBT(tag_id))?,
            };
            vec.push(value);
        }
        Ok(vec)
    }

    fn next_compound(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<NbtTag>, NbtParseError> {
        let mut vec = vec![];
        loop {
            let next = parse_next(iterable)?;
            if next == NbtTag::End {
                break;
            }
            vec.push(next);
        }
        Ok(vec)
    }

    fn next_int_arr(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<i32>, NbtParseError> {
        let size = next_int(iterable)?;
        let mut vec = vec![];
        for _ in 0..size {
            vec.push(next_int(iterable)?)
        }
        Ok(vec)
    }

    fn next_long_arr(iterable: &mut Peekable<Iter<u8>>) -> Result<Vec<i64>, NbtParseError> {
        let size = next_int(iterable)?;
        let mut vec = vec![];
        for _ in 0..size {
            vec.push(next_long(iterable)?)
        }
        Ok(vec)
    }

    pub(super) fn parse_byte(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        Ok(NbtTag::Byte(name, next_byte(iterable)?))
    }

    pub(super) fn parse_short(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        Ok(NbtTag::Short(name, next_short(iterable)?))
    }

    pub(super) fn parse_int(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        Ok(NbtTag::Int(name, next_int(iterable)?))
    }

    pub(super) fn parse_long(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        Ok(NbtTag::Long(name, next_long(iterable)?))
    }

    pub(super) fn parse_float(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        Ok(NbtTag::Float(name, next_float(iterable)?))
    }

    pub(super) fn parse_double(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        Ok(NbtTag::Double(name, next_double(iterable)?))
    }

    pub(super) fn parse_byte_arr(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        Ok(NbtTag::ByteArray(name, next_byte_arr(iterable)?))
    }

    pub(super) fn parse_string(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        let string = next_string(iterable)?;
        Ok(NbtTag::String(name, string))
    }

    pub(super) fn parse_list(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        let vec = next_list(iterable)?;
        Ok(NbtTag::List(name, vec))
    }

    pub(super) fn parse_compound(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        let vec = next_compound(iterable)?;
        Ok(NbtTag::Compound(name, vec))
    }

    pub(super) fn parse_int_array(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        let vec = next_int_arr(iterable)?;
        Ok(NbtTag::IntArray(name, vec))
    }

    pub(super) fn parse_long_array(iterable: &mut Peekable<Iter<u8>>) -> Result<NbtTag, NbtParseError> {
        let name = next_string(iterable)?;
        let vec = next_long_arr(iterable)?;
        Ok(NbtTag::LongArray(name, vec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_byte() {
        let tree = nbt_parser::parse_binary(vec![1, 0, 4, 'b' as u8, 'y' as u8, 't' as u8, 'e' as u8, 0xcc]);
        assert_eq!(tree, NbtTag::Byte("byte".to_string(), 0xccu8 as i8));
    }

    #[test]
    fn parse_short() {
        let tree = nbt_parser::parse_binary(vec![2, 0, 5, 's' as u8, 'h' as u8, 'o' as u8, 'r' as u8, 't' as u8, 0xde, 0xad]);
        assert_eq!(tree, NbtTag::Short("short".to_string(), 0xdeadu16 as i16));
    }

    #[test]
    fn parse_int() {
        let tree = nbt_parser::parse_binary(vec![3, 0, 3, 'i' as u8, 'n' as u8, 't' as u8, 0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(tree, NbtTag::Int("int".to_string(), 0xdeadbeefu32 as i32));
    }

    #[test]
    fn parse_long() {
        let tree = nbt_parser::parse_binary(vec![4, 0, 4, 'l' as u8, 'o' as u8, 'n' as u8, 'g' as u8, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(tree, NbtTag::Long("long".to_string(), 0xdeadbeefdeadbeefu64 as i64));
    }

    #[test]
    fn parse_float() {
        let tree = nbt_parser::parse_binary(vec![5, 0, 5, b'f', b'l', b'o', b'a', b't', 0x46, 0x4f, 0x16, 0x00]);
        assert_eq!(tree, NbtTag::Float("float".to_string(), 13253.5_f32));
    }

    #[test]
    fn parse_double() {
        let tree = nbt_parser::parse_binary(vec![6, 0, 6, b'd', b'o', b'u', b'b', b'l', b'e', 0x5f, 0xbc, 0xe6, 0x7f, 0xb6, 0x5a, 0xfb, 0x65]);
        assert_eq!(tree, NbtTag::Double("double".to_string(), 1.51363604588254730582137744226e153));
    }

    #[test]
    fn parse_byte_array() {
        let tree = nbt_parser::parse_binary(vec![7, 0, 7, 'b' as u8, 'y' as u8, 't' as u8, 'e' as u8, 'a' as u8, 'r' as u8, 'r' as u8, 0, 0, 0, 4, 0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(tree, NbtTag::ByteArray("bytearr".to_string(), vec![0xde_u8 as i8, 0xad_u8 as i8, 0xbe_u8 as i8, 0xef_u8 as i8]));
    }

    #[test]
    fn parse_string() {
        let tree = nbt_parser::parse_binary(vec![8, 0, 7, 's' as u8, 't' as u8, 'r' as u8, 'i' as u8, 'n' as u8, 'g' as u8, '1' as u8, 0, 7, 's' as u8, 't' as u8, 'r' as u8, 'i' as u8, 'n' as u8, 'g' as u8, '2' as u8]);
        assert_eq!(tree, NbtTag::String("string1".to_string(), "string2".to_string()));
    }

    #[test]
    fn parse_list() {
        let tree = nbt_parser::parse_binary(vec![
            // List id
            9,
            // Name length
            0, 4,
            // Name
            'l' as u8, 'i' as u8, 's' as u8, 't' as u8,
            // List contains this type (id)
            2,
            // List length
            0, 0, 0, 5,
            // All the types
            0x11, 0x22,
            0x33, 0x44,
            0x55, 0x66,
            0x77, 0x88,
            0x99, 0xaa,
        ]);
        assert_eq!(tree, NbtTag::List("list".to_string(), vec![
            NbtTag::Short("".to_string(), 0x1122),
            NbtTag::Short("".to_string(), 0x3344),
            NbtTag::Short("".to_string(), 0x5566),
            NbtTag::Short("".to_string(), 0x7788),
            NbtTag::Short("".to_string(), 0x99aa_u16 as i16),
        ]));
    }

    #[test]
    fn parse_compound() {
        let tree = nbt_parser::parse_binary(vec![
            // Compound id
            10,
            // Name length
            0, 4,
            // Name
            'c' as u8, 'o' as u8, 'm' as u8, 'p' as u8,
            // All tags, including ids, names, etc.
            1, 0, 0, 0x22,
            2, 0, 2, b'h', b'i', 0x22, 0x33,
            8, 0, 3, b'l', b'o', b'l', 0, 4, b'l', b'l', b'l', b'l',
            // END tag
            0x00,
        ]);
        assert_eq!(tree, NbtTag::Compound("comp".to_string(), vec![
            NbtTag::Byte("".to_string(), 0x22),
            NbtTag::Short("hi".to_string(), 0x2233),
            NbtTag::String("lol".to_string(), "llll".to_string()),
        ]));
    }

    #[test]
    fn parse_int_array() {
        let tree = nbt_parser::parse_binary(vec![
            11, 0, 6, b'i', b'n', b't', b'a', b'r', b'r',
            // size
            0, 0, 0, 2,
            0, 0, 0, 4, 0xde, 0xad, 0xbe, 0xef
        ]);
        assert_eq!(tree, NbtTag::IntArray("intarr".to_string(), vec![4, 0xdeadbeef_u32 as i32]));
    }

    #[test]
    fn parse_long_array() {
        let tree = nbt_parser::parse_binary(vec![
            12, 0, 7, b'l', b'o', b'n', b'g', b'a', b'r', b'r',
            // size
            0, 0, 0, 3,
            0, 0, 0, 4, 0xde, 0xad, 0xbe, 0xef,
            1, 2, 3, 4, 5, 6, 7, 8,
            9, 10, 11, 12, 13, 14, 15, 16,
        ]);
        assert_eq!(tree, NbtTag::LongArray("longarr".to_string(), vec![
            0x4deadbeef,
            0x0102030405060708,
            0x090a0b0c0d0e0f10,
        ]));
    }
}
