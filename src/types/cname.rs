use std::io;

//use crate::encode::Encode;
use crate::decode::{Decode, DecodeExt};
use crate::types::vlqint32::VLQInt32;

#[derive(Debug, Default, Clone)]
pub struct CName(String);

impl CName {
    const NONE: &str = "None";
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
