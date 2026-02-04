use std::rc::Rc;

use anyhow::Result;
use fnv_rs::{Fnv32, FnvHasher};
use regex::Regex;
use once_cell::sync::Lazy;

use crate::rtti_types::enums::EMaterialVertexFactory;
use crate::rtti_types::structs::SampleStateInfo;

use crate::shader::Shader;

#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub techniques: Vec<Technique>,
}

#[derive(Clone)]
pub struct Technique {
    pub desc: TechniqueDesc,
    pub vs: Option<Rc<Shader>>,
    pub ps: Option<Rc<Shader>>,
    pub vs_samplers: Vec<SampleStateInfo>,
    pub ps_samplers: Vec<SampleStateInfo>,
}

impl PartialEq for Technique {
    fn eq(&self, other: &Self) -> bool {
        self.desc == other.desc
    }
}

impl PartialOrd for Technique {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.desc.partial_cmp(&other.desc)
    }
}


#[derive(PartialEq, Clone)]
pub struct TechniqueDesc {
    pub index: u32,
    pub pass: String,
    pub pass_index: u8,
    pub fallback_index: u8,
    pub vertex_factory: EMaterialVertexFactory,
    pub is_dismembered: bool,
    pub is_discarded: bool,
    pub is_preskinned: bool,
}

impl TechniqueDesc {
    const FLAG_DISMEMBERED: u32 = 0x04;
    const FLAG_DISCARDED: u32   = 0x02;
    const FLAG_PRESKINNED: u32  = 0x01;

    fn has_flag(val: u32, flag: u32) -> bool { val & flag == flag }

    pub fn decode_vf_id(&mut self, id: u32) -> Result<()> {
        let factory_id: u8 = (id >> 3).try_into()?;
        self.vertex_factory = EMaterialVertexFactory::try_from(factory_id)?;
        self.is_dismembered = TechniqueDesc::has_flag(id, TechniqueDesc::FLAG_DISMEMBERED);
        self.is_discarded = TechniqueDesc::has_flag(id, TechniqueDesc::FLAG_DISCARDED);
        self.is_preskinned = TechniqueDesc::has_flag(id, TechniqueDesc::FLAG_PRESKINNED);

        Ok(())
    }

    pub fn encode_vf_id(&self) -> u32 {
        let mut id: u32 = (self.vertex_factory as u32) << 3;

        if self.is_dismembered  { id |= TechniqueDesc::FLAG_DISMEMBERED; }
        if self.is_discarded    { id |= TechniqueDesc::FLAG_DISCARDED;   }
        if self.is_preskinned   { id |= TechniqueDesc::FLAG_PRESKINNED;  }

        id
    }

    pub fn encode_hash(&self) -> u32 {
        let mut hasher = Fnv32::new();
        
        hasher.update(&self.encode_vf_id().to_le_bytes());
        hasher.update(&self.index.to_le_bytes());
        hasher.update(&self.pass.as_bytes());
        hasher.update(&self.pass_index.to_le_bytes());

        hasher.into()
    }

    pub fn encode_string(&self) -> String {
        let mut flag_str: String = String::from("");
        if self.is_discarded { flag_str += "; Discarded"; }
        if self.is_preskinned { flag_str += "; PreSkinned"; }
        if self.is_dismembered { flag_str += "; Dismembered"; }

        format!(
            "CompiledTechnique [Index: {}, Pass '{}', PassIndex: {}, Fallback: {}, RenderStageContext: [ID: {}, VF: {}{}]",
            self.index,
            self.pass,
            self.pass_index,
            self.fallback_index,
            self.encode_vf_id(),
            self.vertex_factory,
            flag_str
        ).to_string()
    }

    pub fn decode_string(input: String) -> Result<Self> {
        static TECH_DESC_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
            r"CompiledTechnique \[Index: (?<index>\d+), Pass '(?<pass>[^']+)', PassIndex: (?<pass_idx>\d+), Fallback: (?<fallback>\d+), RenderStageContext: \[ID: (?<vf_id>\d+), VF: (?<vf>[^\s;\]]+)(?<discard>; Discarded)?(?<preskin>; PreSkinned)?(?<dismem>; Dismembered)?"
            ).unwrap()
        });

        let tokens = TECH_DESC_RE.captures(input.as_str()).unwrap();
        
        Ok(TechniqueDesc {
            index: tokens["index"].parse()?,
            pass: tokens["pass"].to_string(),
            pass_index: tokens["pass_idx"].parse()?,
            fallback_index: tokens["fallback"].parse()?,
            vertex_factory: tokens["vf"].try_into()?,
            is_discarded: tokens.name("discard").is_some(),
            is_preskinned: tokens.name("preskin").is_some(),
            is_dismembered: tokens.name("dismem").is_some(),
        })
    }
}

impl PartialOrd for TechniqueDesc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.encode_vf_id().cmp(&other.encode_vf_id())
            .then(self.index.cmp(&other.index))
            .then(self.pass.cmp(&other.pass))
            .then(self.pass_index.cmp(&other.pass_index))
        )
    }
}

impl std::fmt::Display for TechniqueDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.encode_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_vf_id() {
        // [ID: 229, VF: MeshExtSkinnedLightBlockers; PreSkinned; Dismembered]
        let tech = TechniqueDesc {
            index: 0,
            pass: String::from(""),
            pass_index: 0,
            fallback_index: 0,
            vertex_factory: EMaterialVertexFactory::MeshExtSkinnedLightBlockers,
            is_dismembered: true,
            is_discarded: false,
            is_preskinned: true
        };

        assert_eq!(tech.encode_vf_id(), 229)
    }

    #[test]
    fn encode_hash() {
        // CompiledTechnique [Index: 3, Pass 'renderstage_skin_translucency', PassIndex: 1, Fallback: 0, RenderStageContext: [ID: 222, VF: MeshSkinnedLightBlockers; Discarded; Dismembered]
        let tech = TechniqueDesc {
            index: 3,
            pass: String::from("renderstage_skin_translucency"),
            pass_index: 1,
            fallback_index: 0,
            vertex_factory: EMaterialVertexFactory::MeshSkinnedLightBlockers,
            is_dismembered: true,
            is_discarded: true,
            is_preskinned: false
        };

        assert_eq!(tech.encode_hash(), 0x1FD96A39)
    }

    #[test]
    fn encode_string() {
        let known = "CompiledTechnique [Index: 3, Pass 'renderstage_highlights', PassIndex: 0, Fallback: 0, RenderStageContext: [ID: 55, VF: GarmentMeshExtSkinned; Discarded; PreSkinned; Dismembered]";
        let tech = TechniqueDesc {
            index: 3,
            pass: String::from("renderstage_highlights"),
            pass_index: 0,
            fallback_index: 0,
            vertex_factory: EMaterialVertexFactory::GarmentMeshExtSkinned,
            is_dismembered: true,
            is_discarded: true,
            is_preskinned: true
        };
        assert_eq!(tech.encode_string().as_str(), known);
    }

    #[test]
    fn decode_string() {
        let known = "CompiledTechnique [Index: 3, Pass 'renderstage_skin_translucency', PassIndex: 1, Fallback: 0, RenderStageContext: [ID: 245, VF: GarmentMeshExtSkinnedLightBlockers; PreSkinned; Dismembered]";
        let tech = TechniqueDesc::decode_string(String::from(known)).unwrap();
        assert_eq!(tech.encode_string().as_str(), known);
    }
}