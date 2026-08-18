use eframe::egui;
use kist_core::{open_archive, ArchiveDocument, ArchiveEntry};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

const BG: egui::Color32 = egui::Color32::from_rgb(12, 15, 20);
const SIDEBAR: egui::Color32 = egui::Color32::from_rgb(15, 18, 24);
const PANEL: egui::Color32 = egui::Color32::from_rgb(20, 24, 31);
const PANEL_2: egui::Color32 = egui::Color32::from_rgb(25, 30, 38);
const PANEL_3: egui::Color32 = egui::Color32::from_rgb(30, 35, 44);
const BORDER: egui::Color32 = egui::Color32::from_rgb(43, 49, 60);
const TEXT: egui::Color32 = egui::Color32::from_rgb(238, 241, 246);
const MUTED: egui::Color32 = egui::Color32::from_rgb(143, 151, 166);
const FAINT: egui::Color32 = egui::Color32::from_rgb(102, 111, 126);
const ACCENT: egui::Color32 = egui::Color32::from_rgb(126, 113, 255);
const ACCENT_SOFT: egui::Color32 = egui::Color32::from_rgb(35, 33, 58);
const SUCCESS: egui::Color32 = egui::Color32::from_rgb(113, 201, 94);
const DANGER: egui::Color32 = egui::Color32::from_rgb(244, 93, 94);

#[derive(Default)]
struct KistApp {
    document: Option<ArchiveDocument>,
    current_path: String,
    error: Option<String>,
    search_query: String,
    recent_archives: Vec<PathBuf>,
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
                self.recent_archives.retain(|recent| recent != &path);
                self.recent_archives.insert(0, path);
                self.recent_archives.truncate(6);
                self.document = Some(document);
                self.current_path.clear();
                self.search_query.clear();
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

    fn show_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidebar")
            .exact_width(250.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(SIDEBAR)
                    .inner_margin(egui::Margin::same(16)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    egui::Frame::new()
                        .fill(ACCENT)
                        .corner_radius(7.0)
                        .inner_margin(egui::Margin::symmetric(8, 4))
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("K").strong().color(TEXT));
                        });
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new("Kist").strong().size(20.0).color(TEXT));
                });

                ui.add_space(24.0);
                if sidebar_button(ui, "▣   Open Archive", true).clicked() {
                    self.pick_archive();
                }
                if sidebar_button(ui, "+   Create Archive", false).clicked() {
                    self.error = Some("Archive creation is coming with write support.".to_owned());
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(14.0);
                ui.label(
                    egui::RichText::new("RECENT ARCHIVES")
                        .small()
                        .strong()
                        .color(FAINT),
                );
                ui.add_space(8.0);

                if self.recent_archives.is_empty() {
                    ui.label(
                        egui::RichText::new("Opened archives will appear here.")
                            .small()
                            .color(FAINT),
                    );
                } else {
                    let recent = self.recent_archives.clone();
                    for path in recent {
                        if recent_row(ui, &path).clicked() {
                            self.open_path(path);
                        }
                    }
                }

                if let Some(document) = self.document.as_ref() {
                    ui.add_space(18.0);
                    archive_card(ui, document);
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    if ui
                        .add_sized(
                            [218.0, 36.0],
                            egui::Button::new(
                                egui::RichText::new("⚙   Settings").color(MUTED),
                            )
                            .fill(egui::Color32::TRANSPARENT)
                            .stroke(egui::Stroke::NONE),
                        )
                        .clicked()
                    {
                        self.error = Some("Settings will arrive with persisted preferences.".to_owned());
                    }
                });
            });
    }

    fn show_home(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG))
            .show(ctx, |ui| {
                ui.add_space(92.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Open an archive")
                            .size(30.0)
                            .strong()
                            .color(TEXT),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(
                            "Browse compressed files without turning a simple task into software archaeology.",
                        )
                        .size(15.0)
                        .color(MUTED),
                    );
                    ui.add_space(32.0);

                    egui::Frame::new()
                        .fill(PANEL)
                        .stroke(egui::Stroke::new(1.0_f32, BORDER))
                        .corner_radius(14.0)
                        .inner_margin(egui::Margin::same(30))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width().min(620.0));
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("▣").size(30.0).color(ACCENT));
                                ui.add_space(12.0);
                                ui.label(
                                    egui::RichText::new("Drop an archive here")
                                        .size(18.0)
                                        .strong()
                                        .color(TEXT),
                                );
                                ui.add_space(5.0);
                                ui.label(
                                    egui::RichText::new("or choose a file from your computer")
                                        .color(MUTED),
                                );
                                ui.add_space(18.0);
                                if ui
                                    .add_sized(
                                        [150.0, 38.0],
                                        egui::Button::new(
                                            egui::RichText::new("Open Archive")
                                                .strong()
                                                .color(TEXT),
                                        )
                                        .fill(ACCENT_SOFT)
                                        .stroke(egui::Stroke::new(1.0_f32, ACCENT))
                                        .corner_radius(8.0),
                                    )
                                    .clicked()
                                {
                                    self.pick_archive();
                                }
                            });
                        });
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
        let mut rows = visible_rows(&document.entries, &self.current_path);
        if !self.search_query.trim().is_empty() {
            let query = self.search_query.to_lowercase();
            rows.retain(|row| row.name.to_lowercase().contains(&query));
        }

        egui::TopBottomPanel::top("tab_bar")
            .exact_height(46.0)
            .frame(
                egui::Frame::new()
                    .fill(SIDEBAR)
                    .inner_margin(egui::Margin::symmetric(14, 6)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [220.0, 34.0],
                        egui::Button::new(
                            egui::RichText::new(format!("▣   {archive_name}   ×")).color(TEXT),
                        )
                        .fill(PANEL)
                        .stroke(egui::Stroke::new(1.0_f32, BORDER))
                        .corner_radius(8.0),
                    );
                    if ui.button("+").clicked() {
                        self.pick_archive();
                    }
                });
            });

        egui::TopBottomPanel::top("navigation")
            .exact_height(74.0)
            .frame(
                egui::Frame::new()
                    .fill(PANEL)
                    .inner_margin(egui::Margin::symmetric(18, 15)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("←").clicked() {
                        self.current_path = parent_path(&self.current_path);
                    }
                    ui.add_enabled(false, egui::Button::new("→"));
                    ui.separator();
                    breadcrumbs(ui, &archive_name, &mut self.current_path);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .hint_text("Search in archive...")
                                .desired_width(250.0)
                                .margin(egui::Margin::symmetric(12, 8)),
                        );
                    });
                });
            });

        egui::TopBottomPanel::top("actions")
            .exact_height(62.0)
            .frame(
                egui::Frame::new()
                    .fill(BG)
                    .inner_margin(egui::Margin::symmetric(18, 11)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if action_button(ui, "+  Add", false).clicked() {
                        self.error = Some("Adding files is coming with write support.".to_owned());
                    }
                    if action_button(ui, "▣  Extract", true).clicked() {
                        self.error = Some("Extraction is the next functional slice.".to_owned());
                    }
                    if action_button(ui, "✓  Test", false).clicked() {
                        self.error = Some("Integrity testing is coming next.".to_owned());
                    }
                    if danger_button(ui, "⌫  Delete").clicked() {
                        self.error = Some("Entry deletion is not implemented yet.".to_owned());
                    }
                    if action_button(ui, "✎  Rename", false).clicked() {
                        self.error = Some("Entry rename is not implemented yet.".to_owned());
                    }
                    action_button(ui, "•••  More", false);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("▦   ☷").size(18.0).color(ACCENT));
                    });
                });
            });

        egui::TopBottomPanel::bottom("status")
            .exact_height(34.0)
            .frame(
                egui::Frame::new()
                    .fill(PANEL)
                    .inner_margin(egui::Margin::symmetric(18, 8)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("●").color(SUCCESS).size(10.0));
                    ui.label(egui::RichText::new("Ready").small().color(MUTED));
                });
            });

        egui::TopBottomPanel::bottom("summary")
            .exact_height(112.0)
            .frame(
                egui::Frame::new()
                    .fill(BG)
                    .inner_margin(egui::Margin::symmetric(18, 10)),
            )
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    egui::Frame::new()
                        .fill(PANEL)
                        .stroke(egui::Stroke::new(1.0_f32, BORDER))
                        .corner_radius(10.0)
                        .inner_margin(egui::Margin::same(16))
                        .show(&mut columns[0], |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new(
                                        "Drop files or folders here to add to this archive",
                                    )
                                    .strong()
                                    .color(TEXT),
                                );
                                ui.label(
                                    egui::RichText::new("Write support is coming next")
                                        .small()
                                        .color(FAINT),
                                );
                            });
                        });
                    egui::Frame::new()
                        .fill(PANEL)
                        .stroke(egui::Stroke::new(1.0_f32, BORDER))
                        .corner_radius(10.0)
                        .inner_margin(egui::Margin::same(14))
                        .show(&mut columns[1], |ui| {
                            ui.columns(4, |stats| {
                                summary_stat(&mut stats[0], &summary.entries.to_string(), "files");
                                summary_stat(
                                    &mut stats[1],
                                    &format_bytes(summary.original_size),
                                    "original",
                                );
                                summary_stat(
                                    &mut stats[2],
                                    &format_bytes(summary.compressed_size),
                                    "packed",
                                );
                                summary_stat(
                                    &mut stats[3],
                                    &format!("{:.0}%", summary.savings_percent().max(0.0)),
                                    "smaller",
                                );
                            });
                        });
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG)
                    .inner_margin(egui::Margin::symmetric(18, 0)),
            )
            .show(ctx, |ui| {
                egui::Frame::new()
                    .fill(PANEL)
                    .stroke(egui::Stroke::new(1.0_f32, BORDER))
                    .corner_radius(10.0)
                    .show(ui, |ui| {
                        table_header(ui);
                        ui.separator();
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                if !self.current_path.is_empty() {
                                    let parent = BrowserRow {
                                        name: ".. Parent folder".to_owned(),
                                        path: parent_path(&self.current_path),
                                        size: 0,
                                        compressed_size: None,
                                        is_directory: true,
                                    };
                                    if row_ui(ui, &parent).double_clicked() {
                                        self.current_path = parent.path;
                                    }
                                }
                                for row in rows {
                                    let response = row_ui(ui, &row);
                                    if row.is_directory && response.double_clicked() {
                                        self.current_path = row.path.clone();
                                    }
                                }
                            });
                    });
            });
    }
}

impl eframe::App for KistApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_drop(ctx);
        self.show_sidebar(ctx);
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
                .frame(
                    egui::Frame::new()
                        .fill(PANEL_2)
                        .stroke(egui::Stroke::new(1.0_f32, BORDER))
                        .corner_radius(12.0)
                        .inner_margin(egui::Margin::same(20)),
                )
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

fn sidebar_button(ui: &mut egui::Ui, label: &str, active: bool) -> egui::Response {
    ui.add_sized(
        [218.0, 42.0],
        egui::Button::new(egui::RichText::new(label).color(TEXT))
            .fill(if active { ACCENT_SOFT } else { egui::Color32::TRANSPARENT })
            .stroke(if active {
                egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(56, 51, 92))
            } else {
                egui::Stroke::NONE
            })
            .corner_radius(8.0),
    )
}

fn recent_row(ui: &mut egui::Ui, path: &Path) -> egui::Response {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("Archive");
    let parent = path
        .parent()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(6, 6))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("▣").color(MUTED));
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(name).color(TEXT).size(13.0));
                    ui.label(egui::RichText::new(parent).color(FAINT).small());
                });
            });
        })
        .response
        .interact(egui::Sense::click())
}

fn archive_card(ui: &mut egui::Ui, document: &ArchiveDocument) {
    let summary = &document.summary;
    let name = summary
        .path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("Archive");
    egui::Frame::new()
        .fill(PANEL)
        .stroke(egui::Stroke::new(1.0_f32, BORDER))
        .corner_radius(10.0)
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(name).strong().color(TEXT));
            ui.label(
                egui::RichText::new(summary.format.label())
                    .small()
                    .color(MUTED),
            );
            ui.add_space(12.0);
            compact_stat(ui, "Files", summary.entries.to_string());
            compact_stat(ui, "Size", format_bytes(summary.original_size));
            compact_stat(ui, "Packed", format_bytes(summary.compressed_size));
            compact_stat(
                ui,
                "Compression",
                format!("{:.0}% smaller", summary.savings_percent().max(0.0)),
            );
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(7.0);
            ui.label(egui::RichText::new("✓  Archive loaded").small().color(SUCCESS));
        });
}

fn compact_stat(ui: &mut egui::Ui, label: &str, value: String) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).small().color(FAINT));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(value).small().color(MUTED));
        });
    });
}

fn breadcrumbs(ui: &mut egui::Ui, archive_name: &str, current_path: &mut String) {
    if ui.button(format!("▣  {archive_name}")).clicked() {
        current_path.clear();
    }
    let segments: Vec<String> = current_path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(str::to_owned)
        .collect();
    let mut built = String::new();
    for segment in segments {
        ui.label(egui::RichText::new("›").color(FAINT));
        if !built.is_empty() {
            built.push('/');
        }
        built.push_str(&segment);
        let target = built.clone();
        if ui.button(&segment).clicked() {
            *current_path = target;
        }
    }
}

fn action_button(ui: &mut egui::Ui, label: &str, accent: bool) -> egui::Response {
    ui.add_sized(
        [104.0, 38.0],
        egui::Button::new(
            egui::RichText::new(label).color(if accent { ACCENT } else { TEXT }),
        )
        .fill(if accent { ACCENT_SOFT } else { PANEL_2 })
        .stroke(if accent {
            egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(73, 65, 129))
        } else {
            egui::Stroke::new(1.0_f32, BORDER)
        })
        .corner_radius(7.0),
    )
}

fn danger_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add_sized(
        [104.0, 38.0],
        egui::Button::new(egui::RichText::new(label).color(DANGER))
            .fill(PANEL_2)
            .stroke(egui::Stroke::new(1.0_f32, BORDER))
            .corner_radius(7.0),
    )
}

fn summary_stat(ui: &mut egui::Ui, value: &str, label: &str) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(value).size(20.0).strong().color(TEXT));
        ui.label(egui::RichText::new(label).small().color(FAINT));
    });
}

fn table_header(ui: &mut egui::Ui) {
    egui::Frame::new()
        .fill(PANEL_3)
        .inner_margin(egui::Margin::symmetric(16, 10))
        .show(ui, |ui| {
            ui.columns(5, |columns| {
                columns[0].label(egui::RichText::new("Name  ↕").small().strong());
                columns[1].label(egui::RichText::new("Size").small().strong());
                columns[2].label(egui::RichText::new("Packed Size").small().strong());
                columns[3].label(egui::RichText::new("Ratio").small().strong());
                columns[4].label(egui::RichText::new("Type").small().strong());
            });
        });
}

fn row_ui(ui: &mut egui::Ui, row: &BrowserRow) -> egui::Response {
    let ratio = match row.compressed_size {
        Some(packed) if row.size > 0 => {
            let saved = 100.0 - (packed as f64 / row.size as f64 * 100.0);
            format!("{:.0}%", saved.max(0.0))
        }
        _ => "—".to_owned(),
    };
    let response = egui::Frame::new()
        .fill(PANEL)
        .inner_margin(egui::Margin::symmetric(16, 9))
        .show(ui, |ui| {
            ui.columns(5, |columns| {
                columns[0].label(
                    egui::RichText::new(format!(
                        "{}   {}",
                        if row.is_directory { "■" } else { "□" },
                        row.name
                    ))
                    .color(TEXT),
                );
                columns[1].label(egui::RichText::new(if row.is_directory {
                    "—".to_owned()
                } else {
                    format_bytes(row.size)
                }));
                columns[2].label(egui::RichText::new(if row.is_directory {
                    "—".to_owned()
                } else {
                    row.compressed_size
                        .map(format_bytes)
                        .unwrap_or_else(|| "—".to_owned())
                }));
                columns[3].label(
                    egui::RichText::new(ratio)
                        .color(if row.is_directory { FAINT } else { SUCCESS }),
                );
                columns[4].label(egui::RichText::new(if row.is_directory {
                    "Folder"
                } else {
                    file_type(&row.name)
                }));
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

fn parent_path(current_path: &str) -> String {
    current_path
        .rsplit_once('/')
        .map(|(parent, _)| parent.to_owned())
        .unwrap_or_default()
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
    visuals.widgets.hovered.bg_fill = PANEL_3;
    visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(62, 69, 82));
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(41, 46, 57);
    visuals.selection.bg_fill = egui::Color32::from_rgb(64, 56, 124);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, ACCENT);
    visuals.override_text_color = Some(TEXT);
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 7.0);
    style.spacing.button_padding = egui::vec2(12.0, 7.0);
    ctx.set_style(style);
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Kist")
            .with_inner_size([1380.0, 850.0])
            .with_min_inner_size([980.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Kist",
        options,
        Box::new(|cc| Ok(Box::new(KistApp::new(cc)))),
    )
}
