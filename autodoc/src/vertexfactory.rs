use std::collections::HashSet;
use std::path::PathBuf;

use serde::{de::Error, Deserialize, Serialize};

use shaderpunk::rtti_types::enums::EMaterialVertexFactory;
use shaderpunk::hashmap::CNameKey32;

use crate::link::{Link, LinkList, Linkable};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VertexFactoryVM {
    pub id: VertexFactoryEnum,
    pub desc: String,
    pub layout: String,
    #[serde(skip)]
    pub mat_set: HashSet<CNameKey32>,
    #[serde(default)]
    pub mats: LinkList,
}

impl VertexFactoryVM {
    pub fn get_folder() -> PathBuf {
        PathBuf::from("vertexfactory/")
    }

    pub fn get_template_path() -> PathBuf {
        VertexFactoryVM::get_folder().join(PathBuf::from("_template.md"))
    }

    pub fn get_path_from_enum(e: EMaterialVertexFactory) -> PathBuf {
        VertexFactoryVM::get_folder().join(PathBuf::from(
            format!("{:02}_{}", e as u32, e.to_string())
        ))
    }

    pub fn get_path(&self) -> PathBuf {
        VertexFactoryVM::get_path_from_enum(self.id.into())
    }
}

impl Linkable<VertexFactoryVM> for VertexFactoryVM {
    fn as_link(self: &Self) -> Link {
        Link {
            name: self.id.to_string(),
            path: self.get_path()
        }
    }
}


#[derive(Copy, Clone)]
pub struct VertexFactoryEnum(EMaterialVertexFactory);

impl Default for VertexFactoryEnum {
    fn default() -> Self {
        Self(EMaterialVertexFactory::Invalid)
    }
}

impl Into<EMaterialVertexFactory> for VertexFactoryEnum {
    fn into(self) -> EMaterialVertexFactory {
        self.0
    }
}

impl ToString for VertexFactoryEnum {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
impl Into<u8> for VertexFactoryEnum {
    fn into(self) -> u8 {
        self.0 as u8
    }
}

impl From<EMaterialVertexFactory> for VertexFactoryEnum {
    fn from(value: EMaterialVertexFactory) -> Self {
        VertexFactoryEnum(value)
    }
}

impl Serialize for VertexFactoryEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for VertexFactoryEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        match EMaterialVertexFactory::try_from(String::deserialize(deserializer)?.as_str()) {
            Ok(e) => Ok(VertexFactoryEnum(e)),
            Err(err) => Err(D::Error::custom(format!("{}", err))),
        }
    }
}
