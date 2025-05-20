
fn main() -> anyhow::Result<()> {

    Ok(())
}

/*
fn bool_2_tick(b: bool) -> &'static str { if b { "✓" } else { " " } }

fn main() -> anyhow::Result<()> {

    let file = load_shader_cache(Path::new("C:\\CP77\\Builds\\2.21\\shader_final.cache"))?;
    let tech_count = file.materials.len();

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

    println!("# Compiled Material Techniques");
    println!("{} total materials.<br>", mat_ord.len());
    println!("{} total compiled techniques.<br>", tech_count);
    println!("");

    for m in mat_ord.iter_mut() {
        m.techniques.sort_unstable_by(|a,b| { a.partial_cmp(b).unwrap() });

        println!("## {}", m.name);
        println!("| Index | Pass | Vertex Factory | Discard | PreSkin | Dismember |");
        println!("|---|---|---|:-:|:-:|:-:|");
        for t in &m.techniques {
            
            println!("| {} | {} | {} | {} | {} | {} |",
                t.desc.index,
                t.desc.pass,
                t.desc.vertex_factory.to_string(),
                bool_2_tick(t.desc.is_discarded),
                bool_2_tick(t.desc.is_preskinned),
                bool_2_tick(t.desc.is_dismembered)
            );
        }
        println!("");
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
*/
