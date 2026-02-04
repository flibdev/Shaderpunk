use std::{fmt, io};

use modular_bitfield::prelude::*;
use chrono::{Datelike, Timelike};

use crate::bundle::encode::Encode;
use crate::bundle::decode::{Decode, DecodeExt};

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

        impl $t {
            #[allow(dead_code)]
            pub fn from<T>(dt: T) -> Self
            where T : Datelike + Timelike {
                let mut s = Self::default();
                s.set_year(dt.year() as u16);
                s.set_month((dt.month()-1) as u8);
                s.set_day((dt.day()-1) as u8);
                s.set_hour(dt.hour() as u8);
                s.set_minute(dt.minute() as u8);
                s.set_second(dt.second() as u8);
                s.set_millis((dt.nanosecond()/1_000_000) as u16);

                s
            }
        }
    };
}

timestamp_impl!(TimestampDT);
timestamp_impl!(TimestampTD);

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use chrono::{DateTime, TimeDelta, TimeZone, Utc};

    use crate::bundle::encode::EncodeExt;

    use super::*;
        
    #[test]
    fn decode_td() {
        let bytes = [ 0x4D, 0xA6, 0x0B, 0x01, 0x00, 0x30, 0x90, 0x7E ];
        let mut reader = Cursor::new(bytes);

        let timestamp: TimestampTD = reader.decode().unwrap();

        assert_eq!(timestamp.year(),  2025);
        assert_eq!(timestamp.month(),    0);
        assert_eq!(timestamp.day(),     12);
        assert_eq!(timestamp.hour(),     4);
        assert_eq!(timestamp.minute(),  11);
        assert_eq!(timestamp.second(),  41);
        assert_eq!(timestamp.millis(), 589);
    }
    
    #[test]
    fn decode_dt() {
        let bytes = [ 0x00, 0x30, 0x90, 0x7E, 0x8B, 0xB5, 0xA6, 0x03 ];
        let mut reader = Cursor::new(bytes);

        let timestamp: TimestampDT = reader.decode().unwrap();

        assert_eq!(timestamp.year(),  2025);
        assert_eq!(timestamp.month(),    0);
        assert_eq!(timestamp.day(),     12);
        assert_eq!(timestamp.hour(),    14);
        assert_eq!(timestamp.minute(),  38);
        assert_eq!(timestamp.second(),  45);
        assert_eq!(timestamp.millis(), 395);
    }

    #[test]
    fn encode_td() {
        let buffer: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(buffer);

        let mut datetime: DateTime<Utc> = Utc.with_ymd_and_hms(2025, 1, 13, 4, 13, 22).unwrap();
        datetime += TimeDelta::milliseconds(682);

        let timestamp: TimestampTD = TimestampTD::from(datetime);

        writer.encode(&timestamp).unwrap();

        assert_eq!(writer.get_ref().len(), 8);
        assert_eq!(&writer.get_ref()[0..8], [ 0xAA, 0x5A, 0x0D, 0x01, 0x00, 0x30, 0x90, 0x7E ]);
    }
    
    #[test]
    fn encode_dt() {
        let buffer: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(buffer);

        let mut datetime: DateTime<Utc> = Utc.with_ymd_and_hms(2025, 1, 13, 4, 12, 41).unwrap();
        datetime += TimeDelta::milliseconds(334);

        let timestamp: TimestampDT = TimestampDT::from(datetime);

        writer.encode(&timestamp).unwrap();

        assert_eq!(writer.get_ref().len(), 8);
        assert_eq!(&writer.get_ref()[0..8], [ 0x00, 0x30, 0x90, 0x7E, 0x4E, 0xA5, 0x0C, 0x01 ]);
    }

}
