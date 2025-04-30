use std::io;
use std::path::Path;

use anyhow::Context;
use vmap::Map;

use types::cname::CName;
use cachefile::CacheFile;
use hashmap::{CNameHashMap32, CNameKey32};

pub mod cachefile;
pub mod encode;
pub mod decode;
pub mod hashmap;
pub mod types;

fn main() -> anyhow::Result<()> {

    let file = load_shader_cache(Path::new("C:\\CP77\\Builds\\2.21\\shader_final.cache"))?;

    println!("{:?}", file.info);

    //let passthru = HashMap::<u32, u64, PassThruHasher32>::default();
    let mut materials = CNameHashMap32::<String>::default();
    let mut mat_names: Vec<String> = Vec::new();

    for m in file.materials {
        let hashkey = CNameKey32::from_hash((m.hash >> 32).try_into().unwrap());
        let filename = m.name.as_str().split(' ').next().unwrap();
        let namekey = CNameKey32::from_cname(CName::new(filename));

        if !materials.contains_key(&hashkey) {
            mat_names.push(String::from(filename));
            // 3 materials have a mismatch between filename and CName
            if namekey != hashkey {
                materials.insert(hashkey, String::from(filename));
            }
            else {
                materials.insert(namekey, String::from(filename));
            }
        }
    }

    mat_names.sort();

    println!("Material Count: {}", mat_names.len());
    for m in mat_names {
        println!("{}", m);
    }
    
    
    Ok(())
}

fn load_shader_cache(path: &Path) -> anyhow::Result<CacheFile> {
    let (map, _) = Map::with_options()
        .open(path)
        .context("Failed to open shader cache")?;

    let mut reader = io::Cursor::new(map.as_ref());

    CacheFile::load(&mut reader).context("Failed to load shader cache")
}
