use eframe::egui;
mod gui;

fn main() {
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(377.0, 610.0)),
        initial_window_size: Some(egui::vec2(377.0, 610.0)),
        resizable: false,
        icon_data: Some(eframe::IconData {
            rgba: vec![0, 0, 0, 1],
            width: 16,
            height: 16,
        }),
        ..Default::default()
    };
    eframe::run_native(
        "Calc",
        options,
        Box::new(|_egui_context| Box::new(gui::Calc::default())),
    )
}
