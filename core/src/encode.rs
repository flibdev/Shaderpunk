use std::io;

use byteorder::{LittleEndian, WriteBytesExt};

pub trait Encode {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()>;
}

impl Encode for i64 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_i64::<LittleEndian>(*self)
    }
}

impl Encode for i32 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_i32::<LittleEndian>(*self)
    }
}

impl Encode for i16 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_i16::<LittleEndian>(*self)
    }
}

impl Encode for i8 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_i8(*self)
    }
}

impl Encode for u64 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_u64::<LittleEndian>(*self)
    }
}

impl Encode for u32 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_u32::<LittleEndian>(*self)
    }
}

impl Encode for u16 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_u16::<LittleEndian>(*self)
    }
}

impl Encode for u8 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_u8(*self)
    }
}

impl Encode for bool {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_u8(u8::from(*self))
    }
}

impl Encode for f64 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_f64::<LittleEndian>(*self)
    }
}

impl Encode for f32 {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_f32::<LittleEndian>(*self)
    }
}

impl<const N: usize> Encode for [u8; N] {
    #[inline]
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.write_all(self)
    }
}

pub trait EncodeExt: io::Write + Sized {
    #[inline]
    fn encode<A: Encode>(&mut self, value: &A) -> io::Result<()> {
        value.encode(self)
    }


}

impl<O: io::Write> EncodeExt for O {}
