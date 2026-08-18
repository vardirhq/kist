use eframe::egui;
use kist_core::{ArchiveDocument, ArchiveEntry, open_archive};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

const BG: egui::Color32 = egui::Color32::from_rgb(18, 20, 24);
const PANEL: egui::Color32 = egui::Color32::from_rgb(24, 27, 32);
const PANEL_2: egui::Color32 = egui::Color32::from_rgb(30, 34, 40);
const BORDER: egui::Color32 = egui::Color32::from_rgb(49, 55, 64);
const TEXT: egui::Color32 = egui::Color32::from_rgb(235, 238, 242);
const MUTED: egui::Color32 = egui::Color32::from_rgb(143, 151, 163);
const ACCENT: egui::Color32 = egui::Color32::from_rgb(235, 181, 74);

#[derive(Default)]
struct KistApp {
    document: Option<ArchiveDocument>,
    current_path: String,
    error: Option<String>,
}

#[derive(Clone)]
struct BrowserRow {
    name: String,
    path: String,
    size: u64,
    compressed_size: Option<u64>,
    is_directory: bool,
}

impl KistApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_style(&cc.egui_ctx);
        Self::default()
    }

    fn open_path(&mut self, path: PathBuf) {
        match open_archive(&path) {
            Ok(document) => {
                self.document = Some(document);
                self.current_path.clear();
                self.error = None;
            }
            Err(error) => self.error = Some(error.to_string()),
        }
    }

    fn pick_archive(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Archives", &["zip", "7z", "tar", "gz", "zst"])
            .pick_file()
        {
            self.open_path(path);
        }
    }

    fn handle_drop(&mut self, ctx: &egui::Context) {
        let dropped = ctx.input(|input| input.raw.dropped_files.clone());
        if let Some(path) = dropped.into_iter().find_map(|file| file.path) {
            self.open_path(path);
        }
    }

    fn show_top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar")
            .frame(
                egui::Frame::new()
                    .fill(PANEL)
                    .inner_margin(egui::Margin::symmetric(18, 12)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let mark = egui::RichText::new("K").strong().size(18.0).color(BG);
                    egui::Frame::new()
                        .fill(ACCENT)
                        .corner_radius(7.0)
                        .inner_margin(egui::Margin::symmetric(8, 4))
                        .show(ui, |ui| {
                            ui.label(mark);
                        });
                    ui.label(egui::RichText::new("Kist").strong().size(17.0).color(TEXT));
                    ui.add_space(14.0);
                    ui.separator();
                    ui.add_space(8.0);

                    if ui.button("Home").clicked() {
                        self.document = None;
                        self.current_path.clear();
                        self.error = None;
                    }
                    if ui.button("Open archive").clicked() {
                        self.pick_archive();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("Rust archive utility")
                                .color(MUTED)
                                .small(),
                        );
                    });
                });
            });
    }

    fn show_home(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(92.0);
                    ui.label(egui::RichText::new("Archives without the archaeology.").size(31.0).strong().color(TEXT));
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Open, inspect, and work with compressed files in a focused native desktop app.").size(15.0).color(MUTED));
                    ui.add_space(38.0);

                    let available = ui.available_width().min(700.0);
                    egui::Frame::new()
                        .fill(PANEL)
                        .stroke(egui::Stroke::new(1.0_f32, BORDER))
                        .corner_radius(16.0)
                        .inner_margin(egui::Margin::same(38))
                        .show(ui, |ui| {
                            ui.set_width(available - 76.0);
                            ui.vertical_centered(|ui| {
                                ui.add_space(12.0);
                                ui.label(egui::RichText::new("Drop an archive here").size(20.0).strong().color(TEXT));
                                ui.add_space(6.0);
                                ui.label(egui::RichText::new("ZIP browsing works now. More formats are wired into the product roadmap.").color(MUTED));
                                ui.add_space(22.0);
                                if ui.add(egui::Button::new(egui::RichText::new("Choose archive").strong().color(BG)).fill(ACCENT).corner_radius(8.0)).clicked() {
                                    self.pick_archive();
                                }
                                ui.add_space(12.0);
                            });
                        });

                    ui.add_space(26.0);
                    ui.label(egui::RichText::new("ZIP  ·  7z  ·  TAR  ·  TAR.GZ  ·  TAR.ZST").color(MUTED).small());
                });
            });
    }

    fn show_archive(&mut self, ctx: &egui::Context) {
        let Some(document) = self.document.as_ref() else {
            return;
        };
        let summary = document.summary.clone();
        let archive_name = summary
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Archive")
            .to_owned();
        let rows = visible_rows(&document.entries, &self.current_path);

        egui::SidePanel::left("archive_sidebar")
            .exact_width(220.0)
            .frame(
                egui::Frame::new()
                    .fill(PANEL)
                    .inner_margin(egui::Margin::same(18)),
            )
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.label(egui::RichText::new("ARCHIVE").small().strong().color(MUTED));
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(&archive_name)
                        .size(16.0)
                        .strong()
                        .color(TEXT),
                );
                ui.label(
                    egui::RichText::new(summary.format.label())
                        .color(ACCENT)
                        .small(),
                );
                ui.add_space(24.0);
                stat(ui, "Files", summary.entries.to_string());
                stat(ui, "Unpacked", format_bytes(summary.original_size));
                stat(ui, "Archive size", format_bytes(summary.compressed_size));
                stat(
                    ui,
                    "Space saved",
                    format!("{:.1}%", summary.savings_percent().max(0.0)),
                );
                ui.add_space(18.0);
                ui.separator();
                ui.add_space(14.0);
                ui.label(egui::RichText::new("Path").small().color(MUTED));
                ui.label(
                    egui::RichText::new(summary.path.display().to_string())
                        .small()
                        .color(TEXT),
                );
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG).inner_margin(egui::Margin::same(0)))
            .show(ctx, |ui| {
                egui::Frame::new().fill(PANEL_2).inner_margin(egui::Margin::symmetric(20, 12)).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Extract").clicked() {
                            self.error = Some("Extraction is the next functional slice; browsing is live now.".to_owned());
                        }
                        if ui.button("Test archive").clicked() {
                            self.error = Some("Integrity testing is coming with extraction support.".to_owned());
                        }
                        ui.separator();
                        ui.label(egui::RichText::new(format!("{} items", rows.len())).color(MUTED));
                    });
                });

                show_breadcrumbs(ui, &mut self.current_path);
                table_header(ui);

                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    for row in rows {
                        let response = row_ui(ui, &row);
                        if row.is_directory && response.double_clicked() {
                            self.current_path = row.path.clone();
                        }
                    }
                });
            });
    }
}

impl eframe::App for KistApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_drop(ctx);
        self.show_top_bar(ctx);

        if self.document.is_some() {
            self.show_archive(ctx);
        } else {
            self.show_home(ctx);
        }

        if let Some(message) = self.error.clone() {
            egui::Window::new("Kist")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.set_min_width(380.0);
                    ui.label(message);
                    ui.add_space(12.0);
                    if ui.button("Close").clicked() {
                        self.error = None;
                    }
                });
        }
    }
}

fn stat(ui: &mut egui::Ui, label: &str, value: String) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(MUTED));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(value).strong().color(TEXT));
        });
    });
    ui.add_space(8.0);
}

fn show_breadcrumbs(ui: &mut egui::Ui, current_path: &mut String) {
    egui::Frame::new()
        .fill(PANEL)
        .inner_margin(egui::Margin::symmetric(20, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(current_path.is_empty(), "Archive")
                    .clicked()
                {
                    current_path.clear();
                }

                let segments: Vec<String> = current_path
                    .split('/')
                    .filter(|segment| !segment.is_empty())
                    .map(str::to_owned)
                    .collect();
                let mut built = String::new();
                for segment in segments {
                    ui.label(egui::RichText::new("/").color(MUTED));
                    if !built.is_empty() {
                        built.push('/');
                    }
                    built.push_str(&segment);
                    let target = built.clone();
                    if ui.selectable_label(false, &segment).clicked() {
                        *current_path = target;
                    }
                }
            });
        });
}

fn table_header(ui: &mut egui::Ui) {
    egui::Frame::new()
        .fill(BG)
        .inner_margin(egui::Margin::symmetric(20, 10))
        .show(ui, |ui| {
            ui.columns(4, |columns| {
                columns[0].label(egui::RichText::new("Name").small().strong().color(MUTED));
                columns[1].label(egui::RichText::new("Type").small().strong().color(MUTED));
                columns[2].label(egui::RichText::new("Size").small().strong().color(MUTED));
                columns[3].label(egui::RichText::new("Packed").small().strong().color(MUTED));
            });
        });
}

fn row_ui(ui: &mut egui::Ui, row: &BrowserRow) -> egui::Response {
    let label = if row.is_directory {
        format!("▸  {}", row.name)
    } else {
        format!("   {}", row.name)
    };
    let response = egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(20, 9))
        .show(ui, |ui| {
            ui.columns(4, |columns| {
                columns[0].label(egui::RichText::new(label).color(TEXT));
                columns[1].label(
                    egui::RichText::new(if row.is_directory {
                        "Folder"
                    } else {
                        file_type(&row.name)
                    })
                    .color(MUTED),
                );
                columns[2].label(
                    egui::RichText::new(if row.is_directory {
                        "—".to_owned()
                    } else {
                        format_bytes(row.size)
                    })
                    .color(MUTED),
                );
                columns[3].label(
                    egui::RichText::new(if row.is_directory {
                        "—".to_owned()
                    } else {
                        row.compressed_size
                            .map(format_bytes)
                            .unwrap_or_else(|| "—".to_owned())
                    })
                    .color(MUTED),
                );
            });
        })
        .response
        .interact(egui::Sense::click());
    ui.separator();
    response
}

fn visible_rows(entries: &[ArchiveEntry], current_path: &str) -> Vec<BrowserRow> {
    let prefix = if current_path.is_empty() {
        String::new()
    } else {
        format!("{}/", current_path.trim_end_matches('/'))
    };
    let mut rows: BTreeMap<String, BrowserRow> = BTreeMap::new();

    for entry in entries {
        let full = normalize_path(&entry.path);
        let Some(rest) = full.strip_prefix(&prefix) else {
            continue;
        };
        if rest.is_empty() {
            continue;
        }

        let mut parts = rest.split('/').filter(|part| !part.is_empty());
        let Some(first) = parts.next() else {
            continue;
        };
        let nested = parts.next().is_some();
        let is_directory = nested || entry.is_directory;
        let path = if current_path.is_empty() {
            first.to_owned()
        } else {
            format!("{current_path}/{first}")
        };

        rows.entry(first.to_owned())
            .and_modify(|row| {
                if !is_directory {
                    row.size = entry.size;
                    row.compressed_size = entry.compressed_size;
                }
            })
            .or_insert_with(|| BrowserRow {
                name: first.to_owned(),
                path,
                size: if is_directory { 0 } else { entry.size },
                compressed_size: if is_directory {
                    None
                } else {
                    entry.compressed_size
                },
                is_directory,
            });
    }

    let mut rows: Vec<_> = rows.into_values().collect();
    rows.sort_by(|a, b| {
        b.is_directory
            .cmp(&a.is_directory)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    rows
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .trim_matches('/')
        .to_owned()
}

fn file_type(name: &str) -> &str {
    Path::new(name)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("File")
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

fn configure_style(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BG;
    visuals.window_fill = PANEL;
    visuals.extreme_bg_color = BG;
    visuals.faint_bg_color = PANEL_2;
    visuals.widgets.inactive.bg_fill = PANEL_2;
    visuals.widgets.inactive.weak_bg_fill = PANEL_2;
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, BORDER);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(38, 43, 50);
    visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(67, 74, 84));
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(43, 48, 56);
    visuals.selection.bg_fill = egui::Color32::from_rgb(92, 72, 34);
    visuals.override_text_color = Some(TEXT);
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.button_padding = egui::vec2(13.0, 7.0);
    ctx.set_style(style);
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Kist")
            .with_inner_size([1120.0, 720.0])
            .with_min_inner_size([820.0, 560.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Kist",
        options,
        Box::new(|cc| Ok(Box::new(KistApp::new(cc)))),
    )
}
