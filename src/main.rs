use eframe::NativeOptions;
use ui::Installer;

mod ui;
mod funcs;
mod constants;

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        "Installer",
        options,
        Box::new(|cc| Box::new(Installer::new(cc)))
    ).expect("Failed to Start Installer");
}
