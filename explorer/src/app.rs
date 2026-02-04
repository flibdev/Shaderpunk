use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::{Result, Context};
use egui_extras::{TableBuilder, Column};

use shaderpunk::hashmap::CNameKey32;
use shaderpunk::manager::Manager;
use shaderpunk::material::Material;
use shaderpunk::bundle::dyn_cache::DynamicCacheFile;

use crate::optimize::optimize_cache;

pub struct App {
    run_once: bool,
    cache_path: Option<PathBuf>,
    manager: Option<Manager>,
    error_msg: Option<String>,
    mat_list: Vec<(String, CNameKey32)>,
    material: Option<Rc<Material>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            run_once: true,
            cache_path: None,
            manager: None,
            error_msg: None,
            mat_list: Vec::new(),
            material: None
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn load_cache(&mut self) -> Result<()> {
        let (map, _) = vmap::Map::with_options()
            .open(self.cache_path.clone().unwrap())
            .context("Failed to open shader cache")?;

        let mut reader = std::io::Cursor::new(map.as_ref());

        let cache = DynamicCacheFile::load(&mut reader).context("Failed to load shader cache")?;

        self.manager = Some(Manager::from_dyn_cache(cache)?);

        self.mat_list.clear();
        self.mat_list.reserve(self.manager.as_ref().unwrap().materials.len());

        for (k,m) in &self.manager.as_ref().unwrap().materials {
            self.mat_list.push((m.name.clone(), k.clone()));
        }
        self.mat_list.sort_by(|a,b| a.0.cmp(&b.0));

        Ok(())
    }

    fn save_cache(&mut self, to: PathBuf) -> Result<()> {
        
        let cache = {
            let (map_read, _) = vmap::Map::with_options()
                .open(self.cache_path.clone().unwrap())
                .context("VMAP: Failed to open shader cache")?;

            let mut reader = std::io::Cursor::new(map_read.as_ref());

            DynamicCacheFile::load(&mut reader)?
        };

        // Poke the cache file
        let opt_cache = optimize_cache(&cache)?;

        
        let map_write = File::create(to)
            .context("Failed to create shader cache file")?;
        
        let mut writer = std::io::BufWriter::new(map_write);

        opt_cache.save(&mut writer).context("Failed to save shader cache")?;

        writer.flush()?;

        Ok(())
    }

    fn show_mat_list(&mut self, ui: &mut egui::Ui) {
        if self.manager.is_none() { return; }

        ui.heading(format!("Materials [{}]", &self.mat_list.len()));
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("mat_grid").show(ui, |ui| {
                for (m, k) in &self.mat_list {
                    if ui.button(m).clicked() {
                        self.material = self.manager.as_ref().unwrap().materials.get(k).cloned();
                    }
                    ui.end_row();
                }
            })
        });

    }

    fn show_mat_info(&mut self, ui: &mut egui::Ui) {
        if self.manager.is_none() { return; }

        if self.material.is_none() {
            ui.centered_and_justified(|ui| {
                ui.label("No shader cache loaded");
            });
        }
        else {
            let mat = self.material.clone().unwrap();
            ui.heading(mat.name.clone());
            ui.separator();

            ui.label(format!("Techiques [{}]", mat.techniques.len()));

            /*egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("tech_grid").show(ui, |ui| {

                    ui.label("Pass");
                    ui.label("Vertex Shader");
                    ui.label("Pixel Shader");
                    ui.end_row();

                    for t in &mat.techniques {
                        ui.label(t.desc.pass.clone());
                        ui.label(match &t.vs {
                            Some(vs) => { format!("{:016x}", vs.hash) },
                            None => format!("None"),
                        });
                        ui.label(match &t.ps {
                            Some(ps) => { format!("{:016x}", ps.hash) },
                            None => format!("None"),
                        });
                        ui.end_row();
                    }
                });
            });*/

            TableBuilder::new(ui)
                .striped(true)
                .column(Column::initial(60.0).resizable(true))
                .column(Column::initial(250.0).resizable(true))
                .column(Column::initial(200.0).resizable(true))
                .column(Column::initial(200.0).resizable(true))
                .column(Column::initial(200.0))
                .header(30.0, |mut header| {
                    header.col(|ui| { ui.heading("Index"); });
                    header.col(|ui| { ui.heading("Pass"); });
                    header.col(|ui| { ui.heading("Vertex Factory"); });
                    header.col(|ui| { ui.heading("Vertex Shader"); });
                    header.col(|ui| { ui.heading("Pixel Shader"); });
                })
                .body(|mut body| {
                    for t in &mat.techniques {
                        let mut vfs: Vec<String> = Vec::new();
                        vfs.push(t.desc.vertex_factory.to_string());
                        if t.desc.is_discarded { vfs.push("[DS]".to_string()); }
                        if t.desc.is_preskinned { vfs.push("[PS]".to_string()); }
                        if t.desc.is_dismembered { vfs.push("[DM]".to_string()); }


                        body.row(30.0, |mut row| {
                            row.col(|ui| { ui.label(format!("{}", t.desc.index)); });
                            row.col(|ui| { ui.label(t.desc.pass.clone()); });
                            row.col(|ui| { ui.label(vfs.join(" ")); });
                            row.col(|ui| { ui.label(match &t.vs {
                                Some(vs) => { format!("{:016x}", vs.hash) },
                                None => format!("None"),
                            });});
                            row.col(|ui| { ui.label(match &t.ps {
                                Some(ps) => { format!("{:016x}", ps.hash) },
                                None => format!("None"),
                            });});
                        });
                    }
                });
                
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.run_once {
            catppuccin_egui::set_theme(&ctx, catppuccin_egui::MACCHIATO);

            self.run_once = false;
        }        
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {

            ui.horizontal(|ui| {
                if ui.button("Load Shader Cache").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.cache_path = Some(path);
                        match self.load_cache() {
                            Ok(()) => { self.error_msg = None },
                            Err(e) => { self.error_msg = Some(e.to_string()) }
                        }
                    }
                }
                if self.cache_path.is_some() {
                    if ui.button("Save Shader Cache").clicked() {
                        if let Some(to) = rfd::FileDialog::new().save_file() {
                            match self.save_cache(to) {
                                Ok(()) => { println!("Cache Saved") },
                                Err(e) => { println!("ERROR: {}", e.to_string()) }
                            }
                        }
                    }

                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.label(self.cache_path.as_ref().unwrap().display().to_string())
                    });
                }
            });

        });

        if self.manager.is_none() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    if self.error_msg.is_some() {
                        ui.label(self.error_msg.clone().unwrap());
                    }
                    else {
                        ui.label("No shader cache loaded");
                    }
                });
            });
        }
        else {

            egui::SidePanel::left("mat_list").show(ctx, |ui| { self.show_mat_list(ui); });

            egui::CentralPanel::default().show(ctx, |ui| { self.show_mat_info(ui);  });

        }
    }
}