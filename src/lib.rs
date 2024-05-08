mod types;
mod parser;
mod parse_error;

pub use types::*;
pub use parser::*;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use super::*;

    fn parse_file(file: &str) -> NbtTag {
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("test_files/".to_string() + file);
        let test_compressed = fs::read(test_file).expect("Failed to open test file");

        let tree = nbt_parser::parse_gzip(test_compressed.clone()).unwrap_or_else(|_| nbt_parser::parse_zlib(test_compressed).expect("Could not determine compression format"));

        eprintln!("{file} tree: {:?}", tree);
        tree
    }

    // TODO: Check correctness, at a glance the data looks totally correct
    #[test]
    fn parse_level_dat() {
        parse_file("level.dat");
    }

    #[test]
    fn parse_player_dat() {
        parse_file("player.dat");
    }
}