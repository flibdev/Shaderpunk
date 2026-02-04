use std::collections::HashSet;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use shaderpunk::rtti_types::structs::SampleStateInfo;

use crate::vertexfactory::{VertexFactoryVM, VertexFactoryEnum};
use crate::link::{Link, LinkList, Linkable};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MaterialVM {
    pub name: String,
    pub desc: String,

    #[serde(skip_deserializing)]
    pub techniques: Vec<TechniqueVM>,

    #[serde(default)]
    pub vfs: LinkList,
}

impl MaterialVM {
    pub fn get_folder() -> PathBuf {
        PathBuf::from("material/")
    }

    pub fn get_template_path() -> PathBuf {
        MaterialVM::get_folder().join(PathBuf::from("_template.md"))
    }
}

#[derive(Clone, Default, Serialize)]
pub struct TechniqueVM {
    pub desc: TechniqueDescVM,
    pub vs_samplers: Vec<SampleStateInfo>,
    pub ps_samplers: Vec<SampleStateInfo>,
}

#[derive(Clone, Default, Serialize)]
pub struct TechniqueDescVM {
    pub index: u32,
    pub pass: String,
    pub pass_index: u8,
    pub vertex_factory: Link,
    pub is_dismembered: bool,
    pub is_discarded: bool,
    pub is_preskinned: bool,
}

