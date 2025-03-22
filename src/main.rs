use eframe;

mod interface {
    pub mod gui;
}
use interface::gui::gui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_min_inner_size([377.0, 610.0])
            .with_inner_size([377.0, 610.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Calc",
        options,
        Box::new(|_cc| Ok(Box::new(gui::Calc::default()))),
    )?;
    Ok(())
}
