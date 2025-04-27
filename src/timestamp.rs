use std::{fmt, io};

use modular_bitfield::prelude::*;

use crate::encode::Encode;
use crate::decode::{Decode, DecodeExt};

/// Timestamp stored as Date then Time, as found in redscript and the static cache
#[bitfield]
#[derive(Debug, Default, Clone, Copy)]
pub struct TimestampDT {
    #[skip]
    date_pad: B10,
    pub day: B5,
    pub month: B5,
    pub year: B12,
    pub millis: B10,
    pub second: B6,
    pub minute: B6,
    pub hour: B5,
    #[skip]
    time_pad: B5,
}

/// Timestamp stored as Time then Date, as found in the non-static cache
#[bitfield]
#[derive(Debug, Default, Clone, Copy)]
pub struct TimestampTD {
    pub millis: B10,
    pub second: B6,
    pub minute: B6,
    pub hour: B5,
    
    #[skip]
    padding: B15,

    pub day: B5,
    pub month: B5,
    pub year: B12
}

// I'm sure there's a cleaner way of doing this
macro_rules! timestamp_impl {
    ($t:ty) => {
        impl Encode for $t {
            fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
                output.write_all(&self.into_bytes())
            }
        }
        
        impl Decode for $t {
            fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
                Ok(<$t>::from_bytes(input.decode()?))
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_fmt(format_args!(
                    "{:04}/{:02}/{:02}-{:02}:{:02}:{:02}:{:03}",
                    self.year(),
                    self.month() + 1,
                    self.day() + 1,
                    self.hour(),
                    self.minute(),
                    self.second(),
                    self.millis()
                ))
            }
        }
    };
}

timestamp_impl!(TimestampDT);
timestamp_impl!(TimestampTD);
