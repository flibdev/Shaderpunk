use std::io;

//use crate::encode::Encode;
use crate::decode::{Decode, DecodeExt};

pub struct VLQInt32(i32);

impl VLQInt32 {
    const F_FLAG_SIGN: u8 = 0b1000_0000;
    const F_FLAG_CONT: u8 = 0b0100_0000;
    const F_MASK_DATA: u8 = 0b0011_1111;

    const FLAG_CONT: u8 = 0b1000_0000;
    const MASK_DATA: u8 = 0b0111_1111;
}

fn has_flag(b: u8, f: u8) -> bool { (b & f) == f }

impl Decode for VLQInt32 {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let mut b: u8 = input.decode()?;
        let negative: bool = has_flag(b, VLQInt32::F_FLAG_SIGN);
        let mut value: i32 = (b & VLQInt32::F_MASK_DATA).into();

        // Value larger than 6 bits?
        if has_flag(b, VLQInt32::F_FLAG_CONT) {
            b = input.decode()?;
            value |= i32::from(b & VLQInt32::MASK_DATA) << 6;

            // Value larger than 13 bits?
            if has_flag(b, VLQInt32::FLAG_CONT) {
                b = input.decode()?;
                value |= i32::from(b & VLQInt32::MASK_DATA) << 13;

                // Value larger than 20 bits?
                if has_flag(b, VLQInt32::FLAG_CONT) {
                    b = input.decode()?;
                    value |= i32::from(b & VLQInt32::MASK_DATA) << 20;

                    // Value larger than 27 bits?
                    if has_flag(b, VLQInt32::FLAG_CONT) {
                        b = input.decode()?;
                        value |= i32::from(b & VLQInt32::MASK_DATA) << 27;

                        // Value larger than 27 bits?
                        if has_flag(b, VLQInt32::FLAG_CONT) {
                            b = input.decode()?;
                            value |= i32::from(b & VLQInt32::MASK_DATA) << 27;

                            // Value larger than 34 bits? Overflow
                            if has_flag(b, VLQInt32::FLAG_CONT) {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Invalid VLQInt32 continuation bit set"
                                ));
                            }
                        }
                    }
                }
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
