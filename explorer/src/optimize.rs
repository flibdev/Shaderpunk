use std::io;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

use fnv_rs::{Fnv128, FnvHasher};

use shaderpunk::bundle::dyn_cache::{DynamicCacheFile, ShaderChunk, MaterialChunk};

#[derive(Eq, Clone)]
struct ShaderComparison {
    pub size: usize,
    pub hash: u128,
    pub is_vs: bool,
    pub id: u64,
}

impl PartialEq for ShaderComparison {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
        && self.is_vs == other.is_vs
        && self.hash == other.hash
    }
}
impl Hash for ShaderComparison {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.is_vs.hash(state);
        self.hash.hash(state);
    }
}


pub fn optimize_cache(cache: &DynamicCacheFile) -> io::Result<DynamicCacheFile> {

    let mut mat_map: HashMap<u64, MaterialChunk> = cache.materials.iter().map(|m| (m.hash, m.clone())).collect();
    let old_shader_map: HashMap<u64, ShaderChunk> = cache.shaders.iter().map(|s| (s.hash, s.clone())).collect();
    let mut shader_map: HashMap<u64, ShaderChunk> = HashMap::new();

    let mut shader2mat: HashMap<u64, u64> = HashMap::new();
    for m in &cache.materials {
        if m.vs_hash != 0 {
            shader2mat.insert(m.vs_hash, m.hash);
        }
        if m.ps_hash != 0 {
            shader2mat.insert(m.ps_hash, m.hash);
        }
    }
    
    // calc shaders by size
    let mut shader_comp: Vec<ShaderComparison> = Vec::with_capacity(cache.shaders.len());

    for s in &cache.shaders {
        let mut hasher = Fnv128::new();
        hasher.update(&s.compiled);

        let mat_key = shader2mat.get(&s.hash).unwrap();
        let mat = mat_map.get(mat_key).unwrap();

        let is_vs = mat.vs_hash == s.hash;

        shader_comp.push(ShaderComparison {
            size: s.compiled.len(),
            hash: hasher.into(),
            is_vs,
            id: s.hash
        });
    }
    shader_comp.sort_by(|a,b| a.size.cmp(&b.size).then(a.hash.cmp(&b.hash)));

    let mut comp2shader: HashMap<ShaderComparison, u64> = HashMap::new();

    for comp in shader_comp {
        let mut dupe = false;

        if comp2shader.contains_key(&comp) {
            // possible dupe, actually check
            dupe = true;
        }

        if dupe == true {
            // This is a dupe, update the materials
            let new_shader: u64 = *comp2shader.get(&comp).unwrap();
            let mat_key = shader2mat.get(&comp.id).unwrap();
            let mat = mat_map.get_mut(mat_key).unwrap();
            let (mat_name,_) = mat.name.as_str().split_once(" ").unwrap();
            if comp.is_vs {
                println!("Material {} [{:16X}] VS changing from {:16X} to {:16X}", mat_name, mat.hash, comp.id, new_shader);
                mat.vs_hash = new_shader;
            }
            else {
                println!("Material {} [{:16X}] PS changing from {:16X} to {:16X}", mat_name, mat.hash, comp.id, new_shader);
                mat.ps_hash = new_shader;
            }
        }
        else {
            // Not a dupe, add to hashmap
            comp2shader.insert(comp.clone(), comp.id);
            shader_map.insert(comp.id, old_shader_map.get(&comp.id).unwrap().clone());
        }
    }


    Ok(DynamicCacheFile {
        info: cache.info.clone(),
        shaders: shader_map.into_values().collect(),
        materials: mat_map.into_values().collect(),
        params: cache.params.clone(),
        timestamps: cache.timestamps.clone(),
        includes: cache.includes.clone()
    })
}