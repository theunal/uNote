mod app;
mod crypto;
mod db;
mod models;
mod settings;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 650.0])
            .with_min_inner_size([600.0, 400.0])
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "uNote",
        options,
        Box::new(|_cc| Ok(Box::new(app::NoteApp::new()))),
    )
}
