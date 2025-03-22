pub mod gui {
    use std::f32::NAN;

    use chrono::Local;
    use eframe::egui::{self, *};

    #[derive(Default)]
    pub struct Calc {}

    pub fn custom_window_frame(
        _app: &mut Calc,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        title: &str,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) {
        let text_color = ctx.style().visuals.text_color();
        let titlebar_height = 28.0;

        CentralPanel::default()
            .frame(Frame::none()) // Changed Frame::NONE to Frame::none()
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let painter = ui.painter();

                // Paint the frame:
                painter.rect(
                    rect.shrink(1.0),
                    10.0,
                    ctx.style().visuals.window_fill(),
                    Stroke::new(1.0, text_color),
                    StrokeKind::Middle,
                );

                // Paint the title:
                painter.text(
                    rect.center_top() + vec2(0.0, titlebar_height / 2.0),
                    Align2::CENTER_CENTER,
                    title,
                    FontId::proportional(titlebar_height * 0.8),
                    text_color,
                );

                // Paint the line under the title:
                painter.line_segment(
                    [
                        rect.left_top() + vec2(2.0, titlebar_height),
                        rect.right_top() + vec2(-2.0, titlebar_height),
                    ],
                    Stroke::new(1.0, text_color),
                );

                // Drag to move window:
                let title_bar_rect = {
                    let mut rect = rect;
                    rect.max.y = rect.min.y + titlebar_height;
                    rect
                };
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::drag()); // Changed to Sense::drag(), removed unused response

                // Add the close button:
                let close_response = ui.put(
                    Rect::from_min_max(pos2(345.0, 0.0), rect.right_top()),
                    Button::new(RichText::new("❌").size(titlebar_height - 4.0)).frame(false),
                );
                if close_response.clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close); // Replaced frame.close()
                }

                // Contents:
                let content_rect = {
                    let mut rect = rect;
                    rect.min.y = title_bar_rect.max.y;
                    rect
                }
                .shrink(4.0);
                let mut content_ui = ui.child_ui(content_rect, *ui.layout(), None); // Added Id::new("content")
                add_contents(&mut content_ui);
            });
    }

    impl eframe::App for Calc {
        fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
            egui::Rgba::TRANSPARENT.to_array() // Updated to return [f32; 4]
        }

        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            // Removed unused centering logic since MoveTo isn’t available
            custom_window_frame(self, ctx, frame, "Calc", |ui| {
                ui.separator();

                let calculation: String = String::from("0");

                ui.add_sized(
                    vec2(ui.available_width(), 144.0),
                    egui::Label::new(
                        RichText::new(&calculation)
                            .color(Color32::GREEN)
                            .size(34.0)
                            .strong(),
                    ),
                );

                ui.separator();

                ui.horizontal(|ui| {
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("1").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("2").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(97.3, 55.0),
                        egui::Button::new(RichText::new("+").strong().size(21.0)),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("3").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("4").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(97.3, 55.0),
                        egui::Button::new(RichText::new("-").strong().size(21.0)),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("5").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("6").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(97.3, 55.0),
                        egui::Button::new(RichText::new("*").strong().size(21.0)),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("7").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("8").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(97.3, 55.0),
                        egui::Button::new(RichText::new("/").strong().size(21.0)),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("9").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(127.3, 55.0),
                        egui::Button::new(RichText::new("0").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(44.5, 55.0),
                        egui::Button::new(RichText::new("(").strong().size(21.0)),
                    );
                    ui.add_sized(
                        vec2(44.5, 55.0),
                        egui::Button::new(RichText::new(")").strong().size(21.0)),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_sized(
                        vec2(ui.available_width(), 89.0),
                        egui::Button::new(RichText::new("=").strong().size(34.0)),
                    );
                });

                ui.separator();

                ui.add_sized(
                    ui.available_size(),
                    egui::Label::new(RichText::new(format!(
                        "{}",
                        Local::now().format("%Y-%m-%d %H:%M:%S %z")
                    ))),
                );
            });
        }
    }
}
