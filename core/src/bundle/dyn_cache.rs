use std::io;

//use hashbrown::HashMap;

use chrono::Utc;

use crate::bundle::decode::{Decode, DecodeExt};
use crate::bundle::encode::{Encode, EncodeExt};
use crate::rtti_types::cname::CName;
use crate::rtti_types::structs::SampleStateInfo;
use crate::rtti_types::timestamp::TimestampTD;


pub struct DynamicCacheFile {
    pub info: InfoBlock,
    pub shaders: Vec<ShaderChunk>,
    pub materials: Vec<MaterialChunk>,
    pub params: Vec<ParamsChunk>,
    pub timestamps: Vec<TimestampChunk>,
    pub includes: Vec<IncludesChecksumChunk>,
}

impl DynamicCacheFile {
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
        // Material Techniques

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
        if input.stream_position().unwrap() != info.include_offset {
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


        let cache = DynamicCacheFile {
            info,
            shaders,
            materials,
            params,
            timestamps,
            includes
        };
        Ok(cache)
    }

    pub fn save<O: io::Write + io::Seek>(&self, output: &mut O) -> io::Result<()> {
        
        let mut info: InfoBlock = InfoBlock {
            timestamp: TimestampTD::from(Utc::now()),
            unknown_hash: 0,
            shader_count: self.shaders.len() as u32,
            shader_size: 0,
            material_count: self.materials.len() as u32,
            material_size: 0,
            material_offset: 0,
            param_count: self.params.len() as u32,
            param_size: 0,
            param_offset: 0,
            include_count: self.includes.len() as u32,
            include_size: 0,
            include_offset: 0,
            time_size: 0,
            time_offset: 0,
        };

        //----------------------------------------------------------------------
        // Shaders

        for shader in &self.shaders {
            output.encode(shader)?;
        }

        info.shader_size = output.stream_position()?;

        //----------------------------------------------------------------------
        // Material Techniques

        info.material_offset = info.shader_size; // output.stream_position()?;

        for material in &self.materials {
            output.encode(material)?;
        }
        
        info.material_size = output.stream_position()? - info.material_offset;

        //----------------------------------------------------------------------
        // Material Params

        info.param_offset = output.stream_position()?;

        for param in &self.params {
            output.encode(param)?;
        }

        info.param_size = output.stream_position()? - info.param_offset;

        //----------------------------------------------------------------------
        // Material Timestamps

        info.time_offset = output.stream_position()?;
        
        let timestamp_count: u32 = self.timestamps.len() as u32;
        output.encode(&timestamp_count)?;

        for timestamp in &self.timestamps {
            output.encode(timestamp)?;
        }

        info.time_size = output.stream_position()? - info.time_offset;

        //----------------------------------------------------------------------
        // Include Checksums

        info.include_offset = output.stream_position()?;
        
        let includes_count: u32 = self.includes.len() as u32;
        output.encode(&includes_count)?;

        for include in &self.includes {
            output.encode(include)?;
        }

        info.include_size = output.stream_position()? - info.include_offset;

        //----------------------------------------------------------------------
        // Info block

        output.encode(&info)?;

        Ok(())
    }
}


#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
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

    include_count: u32,
    include_size: u64,
    include_offset: u64,
    
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

        // Some kind of file hash? Checksum?
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
            include_count: path_count,
            include_offset: path_offset,
            include_size: path_size,
            time_size,
            time_offset,
        })
    }
}

impl Encode for InfoBlock {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.encode(&self.shader_count)?;
        output.encode(&self.material_count)?;
        output.encode(&self.param_count)?;

        output.encode(&self.unknown_hash)?;
        output.encode(&self.timestamp)?;

        output.encode(&self.include_count)?;

        output.encode(&self.shader_size)?;
        output.encode(&self.material_size)?;
        output.encode(&self.param_size)?;
        output.encode(&self.include_size)?;
        output.encode(&self.time_size)?;

        output.encode(&self.material_offset)?;
        output.encode(&self.param_offset)?;
        output.encode(&self.time_offset)?;
        output.encode(&self.include_offset)?;

        output.encode(&InfoBlock::MAGIC)?;
        output.encode(&InfoBlock::VERSION)?;

        Ok(())
    }
}

#[derive(Clone)]
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

impl Encode for ShaderChunk {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.encode(&self.hash)?;
        output.encode(&self.params)?;
        let size: u32 = self.compiled.len() as u32;
        output.encode(&size)?;
        output.write_all(&self.compiled)?;

        Ok(())
    }
}

#[derive(Clone)]
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

impl Encode for MaterialChunk {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        let _u32: u32 = 0;
        let _u64: u64 = 0;

        output.encode(&self.hash)?;
        output.encode(&self.name)?;
        
        // Ignored by the game
        output.encode(&_u32)?;

        output.encode(&self.vs_hash)?;
        output.encode(&self.ps_hash)?;
        
        // Ignored by the game
        output.encode(&_u64)?;
        output.encode(&_u64)?;

        output.encode(&self.timestamp)?;

        // Ignored by the game
        output.encode(&_u32)?;

        let vs_count: u32 = self.vs_samplers.len() as u32;
        output.encode(&vs_count)?;
        for vs in &self.vs_samplers {
            output.encode(vs)?;
        }

        let ps_count: u32 = self.ps_samplers.len() as u32;
        output.encode(&ps_count)?;
        for ps in &self.ps_samplers {
            output.encode(ps)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
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

impl Encode for ParamChunk {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.encode(&self.name)?;
        output.encode(&self.value)?;
        output.encode(&self.size)?;

        Ok(())
    }
}

#[derive(Clone)]
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

impl Encode for ParamsChunk {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.encode(&self.hash)?;
        output.encode(&self.mat_mod_mask)?;
        output.encode(&self.param_count)?;

        for p in &self.params {
            output.encode(p)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
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

impl Encode for TimestampChunk {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.encode(&self.hash)?;
        output.encode(&self.timestamp)?;

        Ok(())
    }
}

#[derive(Clone)]
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

impl Encode for IncludesChecksumChunk {
    fn encode<O: io::Write>(&self, output: &mut O) -> io::Result<()> {
        output.encode(&self.path)?;
        output.encode(&self.hash)?;

        Ok(())
    }
}
