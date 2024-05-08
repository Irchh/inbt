mod types;
mod parser;
mod parse_error;

pub use types::*;
pub use parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_byte() {
        let tree = NbtParser::parse_binary(vec![1, 0, 4, 'b' as u8, 'y' as u8, 't' as u8, 'e' as u8, 0xcc]);
        assert_eq!(tree, NbtTag::Byte("byte".to_string(), 0xccu8 as i8));
    }

    #[test]
    fn parse_short() {
        let tree = NbtParser::parse_binary(vec![2, 0, 5, 's' as u8, 'h' as u8, 'o' as u8, 'r' as u8, 't' as u8, 0xde, 0xad]);
        assert_eq!(tree, NbtTag::Short("short".to_string(), 0xdeadu16 as i16));
    }

    #[test]
    fn parse_int() {
        let tree = NbtParser::parse_binary(vec![3, 0, 3, 'i' as u8, 'n' as u8, 't' as u8, 0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(tree, NbtTag::Int("int".to_string(), 0xdeadbeefu32 as i32));
    }

    #[test]
    fn parse_long() {
        let tree = NbtParser::parse_binary(vec![4, 0, 4, 'l' as u8, 'o' as u8, 'n' as u8, 'g' as u8, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(tree, NbtTag::Long("long".to_string(), 0xdeadbeefdeadbeefu64 as i64));
    }

    #[test]
    fn parse_float() {
        let tree = NbtParser::parse_binary(vec![5, 0, 5, b'f', b'l', b'o', b'a', b't', 0x46, 0x4f, 0x16, 0x00]);
        assert_eq!(tree, NbtTag::Float("float".to_string(), 13253.5_f32));
    }

    #[test]
    fn parse_double() {
        let tree = NbtParser::parse_binary(vec![6, 0, 6, b'd', b'o', b'u', b'b', b'l', b'e', 0x5f, 0xbc, 0xe6, 0x7f, 0xb6, 0x5a, 0xfb, 0x65]);
        assert_eq!(tree, NbtTag::Double("double".to_string(), 1.51363604588254730582137744226e153));
    }

    #[test]
    fn parse_byte_array() {
        let tree = NbtParser::parse_binary(vec![7, 0, 7, 'b' as u8, 'y' as u8, 't' as u8, 'e' as u8, 'a' as u8, 'r' as u8, 'r' as u8, 0, 0, 0, 4, 0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(tree, NbtTag::ByteArray("bytearr".to_string(), vec![0xde_u8 as i8, 0xad_u8 as i8, 0xbe_u8 as i8, 0xef_u8 as i8]));
    }

    #[test]
    fn parse_string() {
        let tree = NbtParser::parse_binary(vec![8, 0, 7, 's' as u8, 't' as u8, 'r' as u8, 'i' as u8, 'n' as u8, 'g' as u8, '1' as u8, 0, 7, 's' as u8, 't' as u8, 'r' as u8, 'i' as u8, 'n' as u8, 'g' as u8, '2' as u8]);
        assert_eq!(tree, NbtTag::String("string1".to_string(), "string2".to_string()));
    }

    #[test]
    fn parse_list() {
        let tree = NbtParser::parse_binary(vec![
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
        let tree = NbtParser::parse_binary(vec![
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
        let tree = NbtParser::parse_binary(vec![
            11, 0, 6, b'i', b'n', b't', b'a', b'r', b'r',
            // size
            0, 0, 0, 2,
            0, 0, 0, 4, 0xde, 0xad, 0xbe, 0xef
        ]);
        assert_eq!(tree, NbtTag::IntArray("intarr".to_string(), vec![4, 0xdeadbeef_u32 as i32]));
    }

    #[test]
    fn parse_long_array() {
        let tree = NbtParser::parse_binary(vec![
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
