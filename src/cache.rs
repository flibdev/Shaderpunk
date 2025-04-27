use std::{fmt, io};

use hashbrown::HashMap;

use crate::decode::{Decode, DecodeExt};
use crate::encode::Encode;
use crate::hashmap::PassThruHasher;
use crate::timestamp::TimestampTD;


pub struct CacheFile {
    pub info: InfoBlock,
    pub shaders: Vec<Shader>,
}

impl CacheFile {
    pub fn load<I: io::Read + io::Seek>(input: &mut I) -> io::Result<Self> {
        // Info block is stored as a fixed-size footer
        input.seek(io::SeekFrom::End(-InfoBlock::SIZE))?;
        let info: InfoBlock = input.decode()?;

        // Shaders start at the beginning of the file
        input.seek(io::SeekFrom::Start(0))?;

        let mut shaders: Vec<Shader> = Vec::with_capacity(info.shader_count as usize);
        for _ in 0..info.shader_count {
            shaders.push(input.decode()?);
        }

        let cache = CacheFile {
            info,
            shaders
        };
        Ok(cache)
    }
}



#[derive(Debug, Default)]
pub struct InfoBlock {
    timestamp: TimestampTD,
    unknown_hash: u64,

    shader_count: u32,
    shader_size: u64,
    
    material_count: u32,
    material_size: u64,
    material_offset: u64,

    param_count: u32,
    param_size: u64,
    param_offset: u64,

    path_count: u32,
    path_size: u64,
    path_offset: u64,
    
    time_size: u64,
    time_offset: u64,
}

impl InfoBlock {
    // Magic FourCC       S  H  D  R
    const MAGIC: u32 = 0x53_48_44_52;
    // Known supported file version
    const VERSION: u32 = 10;
    // Fixed size footer block
    const SIZE: i64 = 0x70;
}

impl Decode for InfoBlock {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let shader_count: u32   = input.decode()?;
        let material_count: u32 = input.decode()?;
        let param_count: u32    = input.decode()?;

        // Some kind of file hash?
        let unknown_hash: u64   = input.decode()?;

        // Timestamps stored in same way as redscript cache,
        // but with the date and time u32's reversed
        let timestamp: TimestampTD = input.decode()?;

        // After the timestamp, feels like a later addition
        let path_count: u32     = input.decode()?;

        let shader_size: u64    = input.decode()?;
        let material_size: u64  = input.decode()?;
        let param_size: u64     = input.decode()?;
        let path_size: u64      = input.decode()?;
        let time_size: u64      = input.decode()?;

        let material_offset: u64 = input.decode()?;
        let param_offset: u64   = input.decode()?;
        let time_offset: u64    = input.decode()?;
        let path_offset: u64    = input.decode()?;

        let magic: u32 = input.decode()?;
        let version: u32 = input.decode()?;

        if magic != InfoBlock::MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number"));
        }
        if version != InfoBlock::VERSION {
            return Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported file version"));
        }

        Ok(InfoBlock {
            timestamp,
            unknown_hash,
            shader_count,
            shader_size,
            material_count,
            material_size,
            material_offset,
            param_count,
            param_size,
            param_offset,
            path_count,
            path_offset,
            path_size,
            time_size,
            time_offset,
        })
    }
}



pub struct Shader {
    pub hash: u64,
    unknown: u64,
    pub compiled: Vec<u8>
}

impl Decode for Shader {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let hash: u64 = input.decode()?;
        let unknown: u64 = input.decode()?;
        let size: u32 = input.decode()?;
        let mut compiled: Vec<u8> = vec![0u8; size as usize];
        input.read_exact(&mut compiled)?;

        Ok(Shader {
            hash,
            unknown,
            compiled
        })
    }
}