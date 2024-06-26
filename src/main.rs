use eframe;
use eframe::egui;

mod interface {
    pub mod gui;
}
use interface::gui::gui::*;

fn main() {
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(377.0, 610.0)),
        initial_window_size: Some(egui::vec2(377.0, 610.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Calc",
        options,
        Box::new(|_egui_context| Box::new(Calc::default())),
    )
}
