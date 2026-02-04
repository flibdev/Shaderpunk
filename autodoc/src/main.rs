use std::clone;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs::{DirBuilder, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Context;
use argh::FromArgs;
use handlebars::Handlebars;
use link::{Link, Linkable};
use vmap::Map;

use shaderpunk::bundle::dyn_cache::{DynamicCacheFile, ShaderChunk};
use shaderpunk::hashmap::*;
use shaderpunk::manager::Manager;
use shaderpunk::material::{Material, Technique, TechniqueDesc};
use shaderpunk::rtti_types::cname::CName;
use shaderpunk::rtti_types::enums::EMaterialVertexFactory;

mod link;
mod material;
mod vertexfactory;


use crate::material::{MaterialVM, TechniqueVM, TechniqueDescVM};
use crate::vertexfactory::VertexFactoryVM;

/// github wiki page generator
#[derive(Debug, FromArgs)]
#[argh(help_triggers("-h", "--help"))]
struct Args {
    /// game engine path
    #[argh(option, short='e')]
    engine: PathBuf,
    /// templates path
    #[argh(option, short='t', default = "PathBuf::from(\"autodoc/templates\")")]
    templates: PathBuf,
    /// output path
    #[argh(option, short='o', default = "PathBuf::from(\"output\")")]
    output: PathBuf,
}

fn get_template_data_path(base: &PathBuf, rel: &PathBuf) -> PathBuf {
    base.join(rel).with_extension("toml")
}

fn read_file(path: &PathBuf) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut out: String = String::new();
    file.read_to_string(&mut out)?;
    Ok(out)
}

fn main() -> anyhow::Result<()> {

    let args: Args = argh::from_env();

    let cache = load_shader_cache(&args.engine.join(Path::new("shader_final.cache")))?;
    let man = Manager::from_dyn_cache(cache)?;

    let mat = man.materials.get(&CNameKey32::from(CName::new("metal_base"))).unwrap();

    println!("Name = {}", mat.name);
    println!("Tech.len() = {}", mat.techniques.len());

    /*
    let mut factories: HashMap<EMaterialVertexFactory, VertexFactoryVM> = HashMap::default();

    for vf in 1..34 {
        let e = EMaterialVertexFactory::try_from(vf).unwrap();
        let path = get_template_data_path(&args.templates, &VertexFactoryVM::get_path_from_enum(e));
        
        let mut file = File::open(path)?;
        let mut data: String = String::new();
        file.read_to_string(&mut data)?;

        let toml: VertexFactoryVM = toml::from_str(&data)?;
        factories.insert(e, toml);
    }*/  


    /*
    let mut handlebars = Handlebars::new();

    handlebars.register_partial("link", Link::get_template())?;
    
    handlebars.register_template_string("vf", read_file(&args.templates.join(VertexFactoryVM::get_template_path()))?)?;

    let tmpl = read_file(&args.templates.join(MaterialVM::get_template_path()))?;
    handlebars.register_template_string("mat", tmpl)?;

    
    let data = mat_vms.get(&CNameKey32::from(CName::new("metal_base"))).unwrap();
    
    let output = handlebars.render("mat", data).unwrap();
    println!("{}", output);
    */


    Ok(())
}

fn load_shader_cache(path: &Path) -> anyhow::Result<DynamicCacheFile> {
    let (map, _) = Map::with_options()
        .open(path)
        .context("Failed to open shader cache")?;

    let mut reader = io::Cursor::new(map.as_ref());

    DynamicCacheFile::load(&mut reader).context("Failed to load shader cache")
}

