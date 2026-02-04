use std::rc::Rc;
use anyhow::Result;
use mut_rc::MutRc;

use crate::hashmap::{CNameHashMap32, CNameHashMap64, CNameKey32, CNameKey64};
use crate::bundle::dyn_cache::{DynamicCacheFile, ParamsChunk};

use crate::material::{Material, Technique, TechniqueDesc};
use crate::shader::{Shader, ShaderParam, ShaderParamType, ShaderType};

#[derive(Default)]
pub struct Manager {
    pub materials: CNameHashMap32<Rc<Material>>,
    pub shaders: CNameHashMap64<Rc<Shader>>,
}

impl Manager {

    fn finalize_shader_type(shaders: &CNameHashMap64<MutRc<Shader>>, hash: u64, kind: ShaderType) -> Option<Rc<Shader>> {
        if hash != 0 {
            let s = shaders.get::<CNameKey64>(&hash.into())
                    .expect(format!("Missing {} shader for hash {:016x}", kind.to_string(), hash).as_str());
            let _ = s.with_mut(|s| { s.kind = kind; });
            Some(s.finalize().unwrap())
        }
        else {
            None
        }
    }

    pub fn from_dyn_cache(cache: DynamicCacheFile) -> Result<Manager> {
        let mut materials: CNameHashMap32<MutRc<Material>> = CNameHashMap32::default();
        let mut shaders: CNameHashMap64<MutRc<Shader>> = CNameHashMap64::default();

        // Temporary params hashmap
        let mut params: CNameHashMap64<ParamsChunk> = CNameHashMap64::default();
        for p in cache.params {
            params.insert(p.hash.into(), p);
        }

        // Load bulk shaders
        for s in cache.shaders {
            let params = params.get(&CNameKey64::from(s.params))
                .expect(format!("Could not find matching params for hash {:016x}", s.params).as_str());

            let mut shader: Shader = s.into();
            shader.mat_mod_mask = params.mat_mod_mask;

            for p in &params.params {
                shader.params.push(ShaderParam {
                    name: p.name.clone(),
                    kind: ShaderParamType::try_from(p.size)
                            .expect(format!("Unexpected shader param size: {}", p.size).as_str()),
                    slot: p.value
                });
            }

            shaders.insert(shader.hash.into(), MutRc::new(shader));
        }

        // Load materials
        for m in cache.materials {
            let mat_key: CNameKey32 = ((m.hash >> 32) as u32).into();
            let (mat_name, tech_str) = m.name.as_str().split_once(" ").unwrap();

            if !materials.contains_key(&mat_key) {
                materials.insert(
                    mat_key.clone(),
                    MutRc::new(Material {
                        name: String::from(mat_name),
                        techniques: Vec::new()
                    })
                );
            }

            let tech = Technique {
                desc: TechniqueDesc::decode_string(tech_str.to_string())?,
                vs: Manager::finalize_shader_type(&shaders, m.vs_hash, ShaderType::Vertex),
                ps: Manager::finalize_shader_type(&shaders, m.ps_hash, ShaderType::Pixel),
                vs_samplers: m.vs_samplers,
                ps_samplers: m.ps_samplers,
            };

            _ = materials.get_mut(&mat_key).unwrap().with_mut(|m| { m.techniques.push(tech); });
        }

        for (_, v) in &materials {
            _ = v.with_mut(|m| { m.techniques.sort_by(|a,b| a.desc.partial_cmp(&b.desc).unwrap()); });
        }

        Ok(Manager {
            materials: materials.into_iter().map(|(k,v)| (k, v.finalize().unwrap())).collect(),
            shaders: shaders.into_iter().map(|(k,v)| (k, v.finalize().unwrap())).collect(),
        })
    }
}