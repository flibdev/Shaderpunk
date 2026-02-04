
use app::App;

pub mod app;
pub mod optimize;

fn main() -> eframe::Result {

    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0]),
        
        ..Default::default()
    };

    eframe::run_native(
        "Shaderpunk Explorer",
        opts,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
