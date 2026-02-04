use enum_try_from::impl_enum_try_from;
use strum_macros::{Display, EnumString};

use crate::bundle::dyn_cache::ShaderChunk;
use crate::rtti_types::cname::CName;
use crate::rtti_types::enums::{EMaterialModifier, EnumError};

#[derive(Clone, Copy, Display, EnumString)]
pub enum ShaderType {
    Unknown = 0,
    Vertex,
    Pixel,
    Compute,
    Raytrace
}

impl_enum_try_from!(
    #[repr(u8)]
    #[derive(Clone, Copy)]
    pub enum ShaderParamType {
        Vector = 1,
        Matrix = 4,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);

#[derive(Clone)]
pub struct Shader {
    pub hash: u64,
    pub kind: ShaderType,
    /// Bitmask of supported EMaterialModifier values
    pub mat_mod_mask: u32,
    pub params: Vec<ShaderParam>,
    pub compiled: Vec<u8>,
}

#[derive(Clone)]
pub struct ShaderParam {
    pub name: CName,
    pub kind: ShaderParamType,
    pub slot: u8,
}



impl Shader {
    fn get_supported_mat_mods(&self) -> Vec<EMaterialModifier> {
        let mut matMods: Vec<EMaterialModifier> = Vec::new();
        for i in 0..32 {
            let flag: u32 = 1 << i;
            if self.mat_mod_mask & flag == flag {
                matMods.push(EMaterialModifier::try_from(i).unwrap());
            }
        }
        matMods
    }
}

impl From<ShaderChunk> for Shader {
    fn from(value: ShaderChunk) -> Self {
        Shader {
            hash: value.hash,
            kind: ShaderType::Unknown,
            mat_mod_mask: 0,
            params: Vec::new(),
            compiled: value.compiled
        }
    }
}
