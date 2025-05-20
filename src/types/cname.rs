use std::hash::Hasher;
use std::io;

use fnv_rs::{Fnv64, FnvHasher};

//use crate::encode::Encode;
use crate::decode::{Decode, DecodeExt};
use crate::types::vlqint32::VLQInt32;

#[derive(Debug, Default, Clone)]
pub struct CName(String);

impl CName {
    const NONE: &str = "None";

    pub fn new(s: &str) -> Self {
        if s.len() == 0 {
            Self(String::from(CName::NONE))
        }
        else {
            Self(String::from(s))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn as_hash64(&self) -> u64 {
        if self.0.len() == 0 || self.0 == CName::NONE {
            0
        }
        else {
            let bytes = self.0.as_bytes();
            let mut hasher = Fnv64::new();
            hasher.write(bytes);
            hasher.into()
        }
    }

    pub fn as_hash32(&self) -> u32 {
        let hash64 = self.as_hash64();
        // CName hash key XOR mapping down to 32 bits
        ((hash64 >> 32) ^ (hash64 & 0xffffffff)) as u32
    }
}

impl std::fmt::Display for CName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Decode for CName {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let prefix: VLQInt32 = input.decode()?;
        let length: i32 = prefix.into();
        
        if length == 0 {
            return Ok(CName(String::from(CName::NONE)));
        }

        // Highest bit determines UTF8 vs UTF16
        // prefix length is in characters, not bytes
        let size: usize = (if length > 0 { length*2 } else { -length }) as usize;
        let mut data: Vec<u8> = vec![0; size];
        input.read_exact(&mut data)?;

        if length > 0 {
            let data16: Vec<u16> = data
                .chunks_exact(2)
                .into_iter()
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .collect();

            Ok(CName(String::from_utf16(&data16).unwrap()))
        }
        else {
            Ok(CName(String::from_utf8(data).unwrap()))
        }
    }
}

impl From<CName> for String {
    fn from(value: CName) -> Self {
        value.0
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    
    #[test]
    fn empty_hash64() {
        assert_eq!(CName::new("").as_hash64(), 0);
    }

    #[test]
    fn empty_hash32() {
        assert_eq!(CName::new("").as_hash32(), 0);
    }

    #[test]
    fn empty_string() {
        assert_eq!(CName::new("").as_str(), CName::NONE);
    }

    #[test]
    fn known_hash64() {
        assert_eq!(CName::new("3d_map_solid").as_hash64(), 0xAF5990DA96BB288F);
    }

    #[test]
    fn known_hash32() {
        assert_eq!(CName::new("3d_map_solid").as_hash32(), 0x39E2B855);
    }

    
    #[test]
    fn decode_empty() {
        let bytes = [ 0x00 ];
        let mut reader = Cursor::new(bytes);
        let cname: CName = reader.decode().unwrap();
        
        assert_eq!(cname.as_str(), CName::NONE);
    }

    #[test]
    fn decode_known() {
        // Known value from shader_final.cache
        let bytes = [
            0x8F, 0x69, 0x6E, 0x63, 0x6C, 0x75, 0x64, 0x65,
            0x5F, 0x68, 0x61, 0x69, 0x72, 0x2E, 0x66, 0x78,
        ];
        let mut reader = Cursor::new(bytes);
        let cname: CName = reader.decode().unwrap();
        
        assert_eq!(cname.as_str(), "include_hair.fx");
    }
}
