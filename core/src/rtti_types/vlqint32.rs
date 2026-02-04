use std::io;

//use crate::encode::Encode;
use crate::bundle::decode::{Decode, DecodeExt};
use crate::bundle::encode::{Encode, EncodeExt};

pub struct VLQInt32(i32);

impl VLQInt32 {
    // First byte consts
    const F_FLAG_SIGN: u8 = 0b1000_0000;
    const F_FLAG_CONT: u8 = 0b0100_0000;
    const F_MASK_DATA: u8 = 0b0011_1111;
    const F_DATA_SHIFT: u8 = 6;
    // Continued byte consts
    const FLAG_CONT: u8 = 0b1000_0000;
    const MASK_DATA: u8 = 0b0111_1111;
    const DATA_SHIFT: u8 = 7;
}

fn has_flag(b: u8, f: u8) -> bool { (b & f) == f }

impl Decode for VLQInt32 {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let mut b: u8 = input.decode()?;
        let negative: bool = has_flag(b, VLQInt32::F_FLAG_SIGN);
        let mut value: i32 = (b & VLQInt32::F_MASK_DATA).into();


        if has_flag(b, VLQInt32::F_FLAG_CONT) {
            let mut shift: u8 = VLQInt32::F_DATA_SHIFT;

            b = input.decode()?;
            value |= i32::from(b & VLQInt32::MASK_DATA) << shift;

            while has_flag(b, VLQInt32::FLAG_CONT) {
                shift += VLQInt32::DATA_SHIFT;

                // Can't store more that 32 bits of data
                if shift > 32 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid VLQInt32 continuation bit set"
                    ))
                }

                b = input.decode()?;
                value |= i32::from(b & VLQInt32::MASK_DATA) << shift;
            }
        }

        if negative {
            value = -value;
        }

        Ok(VLQInt32(value))
    }
}

impl From<VLQInt32> for i32 {
    fn from(value: VLQInt32) -> Self {
        value.0
    }
}

impl From<i32> for VLQInt32 {
    fn from(value: i32) -> Self {
        VLQInt32(value)
    }
}

impl Encode for VLQInt32 {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        let negative: bool = self.0 < 0;
        let mut value: i32 = self.0.abs();
        let mut b: u8 = (value as u8) & VLQInt32::F_MASK_DATA;

        if negative {
            b |= VLQInt32::F_FLAG_SIGN;
        }

        value >>= VLQInt32::F_DATA_SHIFT;
        if value > 0 {
            b |= VLQInt32::F_FLAG_CONT;
        }

        output.encode(&b)?;

        while value > 0 {
            b = (value as u8) & VLQInt32::MASK_DATA;
            value >>= VLQInt32::DATA_SHIFT;

            if value > 0 {
                b |= VLQInt32::FLAG_CONT;
            }

            output.encode(&b)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn decode_zero() {
        let bytes = [ 0x00 ];
        let mut reader = Cursor::new(bytes);
        let vlqi32: VLQInt32 = reader.decode().unwrap();

        assert_eq!(vlqi32.0, 0);
    }

    #[test]
    fn decode_single() {
        let bytes = [ 0x2A ];
        let mut reader = Cursor::new(bytes);
        let vlqi32: VLQInt32 = reader.decode().unwrap();

        assert_eq!(vlqi32.0, 42);
    }

    #[test]
    fn decode_max() {
        let bytes = [ 0x7F, 0xFF, 0xFF, 0xFF, 0x0F ];
        let mut reader = Cursor::new(bytes);
        let vlqi32: VLQInt32 = reader.decode().unwrap();

        assert_eq!(vlqi32.0, 2147483647);
    }

    #[test]
    fn decode_min() {
        let bytes = [ 0xFF, 0xFF, 0xFF, 0xFF, 0x0F ];
        let mut reader = Cursor::new(bytes);
        let vlqi32: VLQInt32 = reader.decode().unwrap();

        assert_eq!(vlqi32.0, -2147483647);
    }

    #[test]
    fn decode_invalid() {
        let bytes = [ 0xFF, 0xFF, 0xFF, 0xFF, 0xFF ];
        let mut reader = Cursor::new(bytes);
        let res: io::Result<VLQInt32> = reader.decode();

        assert!(res.is_err());
    }

    
    #[test]
    fn encode_zero() {
        let buffer: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(buffer);
        let vlqi32: VLQInt32 = VLQInt32(0);

        writer.encode(&vlqi32).unwrap();

        assert_eq!(writer.get_ref().len(), 1);
        assert_eq!(writer.get_ref()[0], 0x00);
    }
    
    #[test]
    fn encode_single() {
        let buffer: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(buffer);
        let vlqi32: VLQInt32 = VLQInt32(42);

        writer.encode(&vlqi32).unwrap();

        assert_eq!(writer.get_ref().len(), 1);
        assert_eq!(writer.get_ref()[0], 0x2A);
    }
    
    #[test]
    fn encode_min() {
        let buffer: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(buffer);
        let vlqi32: VLQInt32 = VLQInt32(-2147483647);

        writer.encode(&vlqi32).unwrap();

        assert_eq!(writer.get_ref().len(), 5);
        assert_eq!(&writer.get_ref()[0..5], &[ 0xFF, 0xFF, 0xFF, 0xFF, 0x0F ]);
    }

    #[test]
    fn encode_max() {
        let buffer: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(buffer);
        let vlqi32: VLQInt32 = VLQInt32(2147483647);

        writer.encode(&vlqi32).unwrap();

        assert_eq!(writer.get_ref().len(), 5);
        assert_eq!(&writer.get_ref()[0..5], &[ 0x7F, 0xFF, 0xFF, 0xFF, 0x0F ]);
    }

}
