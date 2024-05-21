use std::{result::Result, str::{FromStr, from_utf8}, ops::BitAnd, fmt::Display};

/// This struct stores a valid 4-byte PNG chunk type
/// Provides methods that return the chunk type in bytes,
/// check the validity of the entire chunk type,
/// check the special meaning of capitalization for each of the four bytes.
#[derive(PartialEq)]
#[derive(Debug)]
pub struct ChunkType {
    ancillary_bit: bool,
    private_bit: bool,
    reserved_bit: bool,
    safe_to_copy_bit: bool,
    bytes: [u8; 4]
}

fn fifth_bit_to_bool(number: &u8) -> bool {
    number.bitand(32) != 0
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = std::str::from_utf8(&self.bytes).unwrap();
        write!(f, "{}", output)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(bytes: [u8; 4]) -> Result<ChunkType, Self::Error> {
        let (mut ancillary_bit, mut private_bit, mut reserved_bit, mut safe_to_copy_bit) = (false, false, false, false);
        for (index, value) in bytes.iter().enumerate() {
            match index {
                0 => {ancillary_bit = fifth_bit_to_bool(value);},
                1 => {private_bit = fifth_bit_to_bool(value);},
                2 => {reserved_bit = fifth_bit_to_bool(value);},
                3 => {safe_to_copy_bit = fifth_bit_to_bool(value);},
                _ => ()
            }
        }
        Ok(ChunkType {
            ancillary_bit,
            private_bit,
            reserved_bit,
            safe_to_copy_bit,
            bytes
        })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(val: &str) -> Result<ChunkType, Self::Err> {
        let mut bytes : [u8; 4] = [0; 4];
        for (index, char) in val.chars().enumerate() {
            if matches!(char, 'a'..='z') || matches!(char, 'A'..='Z') {
                let mut byte = [0; 1];
                char.encode_utf8(&mut byte);
                bytes[index] = byte[0];
            } else {
                return Err("String input should only contain characters A-Z or a-z");
            }
        }
        Ok(ChunkType{
            ancillary_bit: fifth_bit_to_bool(&bytes[0]),
            private_bit: fifth_bit_to_bool(&bytes[1]),
            reserved_bit: fifth_bit_to_bool(&bytes[2]),
            safe_to_copy_bit: fifth_bit_to_bool(&bytes[3]),
            bytes
        })
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        !self.reserved_bit
    }

    pub fn is_critical(&self) -> bool {
        !self.ancillary_bit
    }

    pub fn is_public(&self) -> bool {
        !self.private_bit
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        !self.reserved_bit
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.safe_to_copy_bit
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        println!("chunk bytes -> {:?}", chunk.bytes());
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
