use std::io;

use anyhow::Error;
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

    pub fn as_hash64(&self) -> Result<u64, Error> {
        if self.0.len() == 0 || self.0 == CName::NONE {
            Ok(0)
        }
        else {
            let bytes = self.0.as_bytes();
            let hash = Fnv64::hash(bytes);
            Ok(u64::from_be_bytes(hash.as_bytes().try_into()?))
        }
    }

    pub fn as_hash32(&self) -> Result<u32, Error> {
        let hash64 = self.as_hash64()?;
        // CName hash key XOR mapping down to 32 bits
        let hash32: u32 = ((hash64 >> 32) ^ (hash64 & 0xffffffff)).try_into()?;
        Ok(hash32)
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
