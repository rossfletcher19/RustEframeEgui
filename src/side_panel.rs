use crate::rustbook_code_blocks::{self};
use eframe::egui;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SidePanelState {
    pub spp1: bool, // spp = side_panel_popup
    pub spp2: bool,
    pub spp3: bool,
    pub language: String,
}

impl SidePanelState {
    pub fn new() -> Self {
        Self {
            spp1: false,
            spp2: false,
            spp3: false,
            language: "rs".into(),
        }
    }
}

pub fn side_panel_ui(ctx: &egui::Context, state: &mut SidePanelState) {
    egui::SidePanel::left("side_nav")
        .default_width(100.0)
        // .exact_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Common Rust Concepts");
            if ui.button("📦 Variables, Mutability & DataTypes").clicked() {
                if state.spp1 {
                    state.spp1 = !state.spp1;
                } else {
                    state.spp1 = true;
                }
            }
            if ui.button("📦 Functions").clicked() {
                if state.spp2 {
                    state.spp2 = !state.spp2;
                } else {
                    state.spp2 = true;
                }
            }
            if ui.button("📦 Comments").clicked() {
                if state.spp3 {
                    state.spp3 = !state.spp3;
                } else {
                    state.spp3 = true;
                }
            }
        });

    if state.spp1 {
        egui::Window::new("📦 Variables, Mutability & DataTypes")
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.label("📦 Variables, Mutability & DataTypes");
                let theme =
                    egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
                let code = rustbook_code_blocks::VARIABLES_MUTABILITY_DATATYPES;
                let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                    let mut layout_job = egui_extras::syntax_highlighting::highlight(
                        ui.ctx(),
                        ui.style(),
                        &theme,
                        text,
                        &state.language,
                    );
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };

                egui::ScrollArea::vertical()
                    .max_height(750.0) // Limit height for scrollability
                    .show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 800.0],
                            egui::TextEdit::multiline(&mut code.to_string())
                                .font(egui::TextStyle::Monospace) // Use monospaced font
                                .code_editor() // Enables code styling
                                .desired_rows(13)
                                .lock_focus(true)
                                .desired_width(f32::INFINITY)
                                .layouter(&mut layouter),
                        );
                    });
                if ui.button("❌ Close").clicked() {
                    state.spp1 = false;
                }
            });
    }

    if state.spp2 {
        egui::Window::new("📦 Functions")
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.label("📦 Functions");
                let theme =
                    egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
                let code = rustbook_code_blocks::FUNCTIONS;
                let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                    let mut layout_job = egui_extras::syntax_highlighting::highlight(
                        ui.ctx(),
                        ui.style(),
                        &theme,
                        text,
                        &state.language,
                    );
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };

                egui::ScrollArea::vertical()
                    .max_height(750.0) // Limit height for scrollability
                    .show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 800.0],
                            egui::TextEdit::multiline(&mut code.to_string())
                                .font(egui::TextStyle::Monospace) // Use monospaced font
                                .code_editor() // Enables code styling
                                .desired_rows(13)
                                .lock_focus(true)
                                .desired_width(f32::INFINITY)
                                .layouter(&mut layouter),
                        );
                    });
                if ui.button("❌ Close").clicked() {
                    state.spp2 = false;
                }
            });
    }
    if state.spp3 {
        egui::Window::new("📦 Comments")
            .resizable(true)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.label("📦 Comments");
                let theme =
                    egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
                let code = rustbook_code_blocks::COMMENTS;
                let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                    let mut layout_job = egui_extras::syntax_highlighting::highlight(
                        ui.ctx(),
                        ui.style(),
                        &theme,
                        text,
                        &state.language,
                    );
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };
                egui::ScrollArea::vertical()
                    .max_height(550.0) // Limit height for scrollability
                    .show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 550.0],
                            egui::TextEdit::multiline(&mut code.to_string())
                                .font(egui::TextStyle::Monospace) // Use monospaced font
                                .code_editor() // Enables code styling
                                .desired_rows(13)
                                .lock_focus(true)
                                .desired_width(f32::INFINITY)
                                .layouter(&mut layouter),
                        );
                    });
                if ui.button("❌ Close").clicked() {
                    state.spp3 = false;
                }
            });
    }
}
