mod types;
mod parser;
mod parse_error;

pub use types::*;
pub use parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_name() {
        let name = NbtParser::parse_name(&mut vec![0, 5, 'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8].iter().peekable()).unwrap();
        assert_eq!(name, "hello".to_string());
    }

    #[test]
    fn parse_byte() {
        let tree = NbtParser::parse_binary(vec![1, 0, 6, 'b' as u8, 'y' as u8, 't' as u8, 'e' as u8, 's' as u8, 's' as u8, 123]);
        assert_eq!(tree, NbtTag::Byte("bytess".to_string(), 123));
    }
}
