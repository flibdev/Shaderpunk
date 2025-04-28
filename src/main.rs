use std::io;
use std::path::Path;

use anyhow::Context;
use vmap::Map;

use cache::CacheFile;

pub mod cache;
pub mod encode;
pub mod decode;
pub mod hashmap;
pub mod types;

fn main() -> anyhow::Result<()> {

    let file = load_shader_cache(Path::new("C:\\CP77\\Builds\\2.21\\shader_final.cache"))?;

    println!("{:?}", file.info);

    println!("Shader[16] = {:#x}", file.shaders[16].hash);

    println!("Material[16] = {:?}", file.materials[16].name);
    
    Ok(())
}

fn load_shader_cache(path: &Path) -> anyhow::Result<CacheFile> {
    let (map, _) = Map::with_options()
        .open(path)
        .context("Failed to open shader cache")?;

    let mut reader = io::Cursor::new(map.as_ref());

    CacheFile::load(&mut reader).context("Failed to load shader cache")
}
