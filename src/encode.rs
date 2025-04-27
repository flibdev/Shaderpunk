use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::io;

use byteorder::{LittleEndian, WriteBytesExt};

pub trait Encode {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()>;
}
