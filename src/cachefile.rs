use std::io;

//use hashbrown::HashMap;

use crate::decode::{Decode, DecodeExt};
use crate::types::cname::CName;
use crate::types::rtti_structs::SampleStateInfo;
//use crate::encode::Encode;
//use crate::hashmap::PassThruHasher;
use crate::types::timestamp::TimestampTD;


pub struct CacheFile {
    pub info: InfoBlock,
    pub shaders: Vec<ShaderChunk>,
    pub materials: Vec<MaterialChunk>,
    pub params: Vec<ParamsChunk>,
    pub timestamps: Vec<TimestampChunk>,
    pub includes: Vec<IncludesChecksumChunk>,
}

impl CacheFile {
    pub fn load<I: io::Read + io::Seek>(input: &mut I) -> io::Result<Self> {
        // Info block is stored as a fixed-size footer
        let info_start = input.seek(io::SeekFrom::End(-InfoBlock::SIZE))?;
        let info: InfoBlock = input.decode()?;

        // Shaders start at the beginning of the file
        input.seek(io::SeekFrom::Start(0))?;

        //----------------------------------------------------------------------
        // Shaders

        let mut shaders: Vec<ShaderChunk> = Vec::with_capacity(info.shader_count as usize);
        for _ in 0..info.shader_count {
            shaders.push(input.decode()?);
        }

        // Sanity check
        if input.stream_position().unwrap() != info.material_offset {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "ShaderChunk size mismatch"));
        }

        //----------------------------------------------------------------------
        // Materials

        let mut materials: Vec<MaterialChunk> = Vec::with_capacity(info.material_count as usize);
        for _ in 0..info.material_count {
            materials.push(input.decode()?);
        }

        // Sanity check
        if input.stream_position().unwrap() != info.param_offset {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "MaterialChunk size mismatch"));
        }

        //----------------------------------------------------------------------
        // Material Params

        let mut params: Vec<ParamsChunk> = Vec::with_capacity(info.param_count as usize);
        for _ in 0..info.param_count {
            params.push(input.decode()?);
        }

        // Sanity check
        if input.stream_position().unwrap() != info.time_offset {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "ParamsChunk size mismatch"));
        }

        //----------------------------------------------------------------------
        // Material Timestamps

        let timestamp_count: u32 = input.decode()?;
        let mut timestamps: Vec<TimestampChunk> = Vec::with_capacity(timestamp_count as usize);
        for _ in 0..timestamp_count {
            timestamps.push(input.decode()?);
        }

        // Sanity check
        if input.stream_position().unwrap() != info.path_offset {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "TimestampChunk size mismatch"));
        }

        //----------------------------------------------------------------------
        // Include Checksums

        let include_count: u32 = input.decode()?;
        let mut includes: Vec<IncludesChecksumChunk> = Vec::with_capacity(include_count as usize);
        for _ in 0..include_count {
            includes.push(input.decode()?);
        }

        // Sanity check
        if input.stream_position().unwrap() != info_start {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "IncludesChecksumChunk size mismatch"));
        }


        let cache = CacheFile {
            info,
            shaders,
            materials,
            params,
            timestamps,
            includes
        };
        Ok(cache)
    }
}


#[allow(dead_code)]
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



pub struct ShaderChunk {
    pub hash: u64,
    pub params: u64,
    pub compiled: Vec<u8>
}

impl Decode for ShaderChunk {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let hash: u64 = input.decode()?;
        let params: u64 = input.decode()?;
        let size: u32 = input.decode()?;
        let mut compiled: Vec<u8> = vec![0u8; size as usize];
        input.read_exact(&mut compiled)?;

        Ok(ShaderChunk {
            hash,
            params,
            compiled
        })
    }
}


pub struct MaterialChunk {
    pub hash: u64,
    pub name: CName,
    pub vs_hash: u64,
    pub ps_hash: u64,
    pub timestamp: TimestampTD,
    pub vs_samplers: Vec<SampleStateInfo>,
    pub ps_samplers: Vec<SampleStateInfo>
}

impl Decode for MaterialChunk {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let mut _u32: u32;
        let mut _u64: u64;

        let hash: u64 = input.decode()?;
        let name: CName = input.decode()?;

        // Ignored by the game
        _u32 = input.decode()?;

        let vs_hash: u64 = input.decode()?;
        let ps_hash: u64 = input.decode()?;

        // Ignored by the game
        _u64 = input.decode()?;
        _u64 = input.decode()?;

        let timestamp: TimestampTD = input.decode()?;
        
        // Ignored by the game
        _u32 = input.decode()?;
        
        let vs_sampler_count: u32 = input.decode()?;
        let mut vs_samplers: Vec<SampleStateInfo> = Vec::new();
        for _ in 0..vs_sampler_count {
            vs_samplers.push(input.decode()?);
        }

        let ps_sampler_count: u32 = input.decode()?;
        let mut ps_samplers: Vec<SampleStateInfo> = Vec::new();
        for _ in 0..ps_sampler_count {
            ps_samplers.push(input.decode()?);
        }

        Ok(MaterialChunk {
            hash,
            name,
            vs_hash,
            ps_hash,
            timestamp,
            vs_samplers,
            ps_samplers
        })

    }
}



pub struct ParamChunk {
    pub name: CName,
    // Value? Lookup? Shader register?
    pub value: u8,
    // Always 1 or 4, memory size? scalar vs vec vs matrix?
    // 4 == Matrix
    pub size: u8,
}

impl Decode for ParamChunk {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let name: CName = input.decode()?;
        let value: u8 = input.decode()?;
        let size: u8 = input.decode()?;

        Ok(ParamChunk { name, value, size })
    }
}

pub struct ParamsChunk {
    pub hash: u64,
    pub mat_mod_mask: u32,
    pub param_count: u32,
    pub params: Vec<ParamChunk>,
}

impl Decode for ParamsChunk {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let hash: u64 = input.decode()?;
        let mat_mod_mask: u32 = input.decode()?;
        let param_count: u32 = input.decode()?;
        let mut params: Vec<ParamChunk> = Vec::with_capacity(param_count as usize);
        for _ in 0..param_count {
            params.push(input.decode()?);
        }

        Ok(ParamsChunk {
            hash,
            mat_mod_mask,
            param_count,
            params
        })
    }
}


pub struct TimestampChunk {
    pub hash: u32,
    pub timestamp: TimestampTD,
}

impl Decode for TimestampChunk {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let hash: u32 = input.decode()?;
        let timestamp: TimestampTD = input.decode()?;

        Ok(TimestampChunk { hash, timestamp })
    }
}

pub struct IncludesChecksumChunk {
    pub path: CName,
    pub hash: u64,
}

impl Decode for IncludesChecksumChunk {
    fn decode<I: io::Read>(input: &mut I) -> io::Result<Self> {
        let path: CName = input.decode()?;
        let hash: u64 = input.decode()?;

        Ok(IncludesChecksumChunk { path, hash })
    }
}

