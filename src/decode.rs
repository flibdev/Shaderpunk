use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

pub trait Decode: Sized {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self>;
}

impl Decode for i64 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_i64::<LittleEndian>()
    }
}

impl Decode for i32 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_i32::<LittleEndian>()
    }
}

impl Decode for i16 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_i16::<LittleEndian>()
    }
}

impl Decode for i8 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_i8()
    }
}

impl Decode for u64 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_u64::<LittleEndian>()
    }
}

impl Decode for u32 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_u32::<LittleEndian>()
    }
}

impl Decode for u16 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_u16::<LittleEndian>()
    }
}

impl Decode for u8 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_u8()
    }
}

impl Decode for bool {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        Ok(input.read_u8()? != 0)
    }
}

impl Decode for f64 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_f64::<LittleEndian>()
    }
}

impl Decode for f32 {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        input.read_f32::<LittleEndian>()
    }
}

impl<const N: usize> Decode for [u8; N] {
    #[inline]
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let mut buf: [u8; N] = [0; N];
        input.read_exact(&mut buf)?;
        Ok(buf)
    }
}


pub trait DecodeExt: io::Read + Sized {
    #[inline]
    fn decode<A: Decode>(&mut self) -> io::Result<A> {
        Decode::decode(self)
    }
}

impl<I: io::Read> DecodeExt for I {}
