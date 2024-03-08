use eframe::{egui::{Vec2, ViewportBuilder}, NativeOptions};
use ui::Installer;

mod ui;
mod funcs;
mod constants;

fn main() {
    let mut options = NativeOptions::default();
    options.viewport = ViewportBuilder::default().with_min_inner_size(Vec2::new(800.0, 600.0));
    eframe::run_native(
        "Installer",
        options,
        Box::new(|cc| Box::new(Installer::new(cc)))
    ).expect("Failed to Start Installer");
}
