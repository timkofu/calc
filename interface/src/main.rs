#![doc(html_no_source)]

use eframe::egui;

#[derive(Default)]
struct Calc {}

impl eframe::App for Calc {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.heading("Calc"));
    }
}
impl Calc {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calc",
        native_options,
        Box::new(|cc| Box::new(Calc::new(cc))),
    )
}
