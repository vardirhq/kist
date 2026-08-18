use eframe::egui;

struct KistApp;

impl KistApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_style(&cc.egui_ctx);
        Self
    }
}

impl eframe::App for KistApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(72.0);
                ui.heading("Kist");
                ui.label("A fast, focused archive utility built entirely in Rust.");
                ui.add_space(28.0);

                egui::Frame::new()
                    .fill(ui.visuals().faint_bg_color)
                    .corner_radius(16.0)
                    .inner_margin(32.0)
                    .show(ui, |ui| {
                        ui.set_min_width(480.0);
                        ui.vertical_centered(|ui| {
                            ui.heading("Drop files or an archive here");
                            ui.add_space(8.0);
                            ui.label("Create a new archive or inspect an existing one.");
                            ui.add_space(20.0);
                            ui.horizontal(|ui| {
                                if ui.button("Create archive").clicked() {
                                    // Archive creation lands in the next implementation slice.
                                }
                                if ui.button("Open archive").clicked() {
                                    // Archive browsing lands in the next implementation slice.
                                }
                            });
                        });
                    });

                ui.add_space(28.0);
                ui.weak("ZIP · 7z · TAR · TAR.GZ · TAR.ZST");
            });
        });
    }
}

fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(16.0, 10.0);
    ctx.set_style(style);
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Kist")
            .with_inner_size([920.0, 640.0])
            .with_min_inner_size([720.0, 520.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Kist",
        options,
        Box::new(|cc| Ok(Box::new(KistApp::new(cc)))),
    )
}
