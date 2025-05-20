use std::io;
use std::path::Path;

use anyhow::Context;
use vmap::Map;

use types::cname::CName;
use types::rtti_enums::EMaterialVertexFactory;
use material::{Material, Technique, TechniqueDesc};
use cachefile::CacheFile;
use hashmap::{CNameHashMap32, CNameKey32};


pub mod cachefile;
pub mod encode;
pub mod decode;
pub mod hashmap;
pub mod types;

pub mod material;

fn bool_2_tick(b: bool) -> &'static str { if b { "✓" } else { " " } }

fn main() -> anyhow::Result<()> {

    let file = load_shader_cache(Path::new("C:\\CP77\\Builds\\2.21\\shader_final.cache"))?;

    //println!("{:?}", file.info);

    let mut materials = CNameHashMap32::<Material>::default();

    for m in file.materials {
        let mat_key: CNameKey32 = ((m.hash >> 32) as u32).into();
        let (mat_name, tech_str) = m.name.as_str().split_once(" ").unwrap();

        if !materials.contains_key(&mat_key) {
            materials.insert(
                mat_key.clone(),
                Material {
                    name: String::from(mat_name),
                    techniques: Vec::new()
                }
            );
        }

        let tech = Technique {
            desc: TechniqueDesc::decode_string(tech_str.to_string())?,
            vs_samplers: m.vs_samplers,
            ps_samplers: m.ps_samplers,
        };

        materials.get_mut(&mat_key).unwrap().techniques.push(tech);
    }

    let mut mat_ord: Vec<Material> = materials.into_values().collect();
    mat_ord.sort_unstable_by(|a,b| { a.name.cmp(&b.name) });

    for m in mat_ord.iter_mut() {
        println!("<details>");
        println!("<summary>{} [{}]</summary>", m.name, m.techniques.len());
        
        m.techniques.sort_unstable_by(|a,b| { a.partial_cmp(b).unwrap() });

        println!("| Index | Pass | ID | Vertex Factory | DC | PS | DM |");
        println!("|------:|------|---:|----------------|----|----|----|");
        for t in &m.techniques {
            
            println!("| {} | {} | {} | {} | {} | {} | {} |",
                t.desc.index,
                t.desc.pass,
                t.desc.encode_vf_id(),
                t.desc.vertex_factory.to_string(),
                bool_2_tick(t.desc.is_discarded),
                bool_2_tick(t.desc.is_preskinned),
                bool_2_tick(t.desc.is_dismembered)
            );
        }
        println!("</details>");
    }

    //let passthru = HashMap::<u32, u64, PassThruHasher32>::default();
    /*
    let mut materials = CNameHashMap32::<String>::default();
    let mut mat_names: Vec<String> = Vec::new();

    for m in file.materials {
        let hashkey: CNameKey32 = ((m.hash >> 32) as u32).into();
        let filename = m.name.as_str().split(' ').next().unwrap();
        let namekey: CNameKey32 = CName::new(filename).into();

        if !materials.contains_key(&hashkey) {
            mat_names.push(String::from(filename));
            // 3 materials have a mismatch between filename and CName
            if namekey != hashkey {
                println!("Hash mismatch: {}", namekey);
                materials.insert(hashkey, String::from(filename));
            }
            else {
                materials.insert(namekey, String::from(filename));
            }
        }
    }

    mat_names.sort();

    for m in mat_names {
        println!("{}", m);
    }
    */

    
    Ok(())
}

fn load_shader_cache(path: &Path) -> anyhow::Result<CacheFile> {
    let (map, _) = Map::with_options()
        .open(path)
        .context("Failed to open shader cache")?;

    let mut reader = io::Cursor::new(map.as_ref());

    CacheFile::load(&mut reader).context("Failed to load shader cache")
}
