use crate::rustbook_code_blocks::{self};
use eframe::egui;

pub fn side_panel_ui(ctx: &egui::Context, spp1: &mut bool, language: &mut String) {
    egui::SidePanel::left("side_nav")
        .exact_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Sidebar");
            if ui.button("📦 Variables, Mutability & DataTypes").clicked() {
                *spp1 = true;
            }
        });

    if *spp1 {
        egui::Window::new("📦 Variables, Mutability & DataTypes").show(ctx, |ui| {
            ui.label("📦 Variables, Mutability & DataTypes");
            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
            let code = rustbook_code_blocks::VARIABLES_MUTABILITY_DATATYPES;

            let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    ui.style(),
                    &theme,
                    text,
                    language,
                );
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            egui::ScrollArea::vertical()
                .max_height(600.0) // Limit height for scrollability
                .show(ui, |ui| {
                    ui.add_sized(
                        [ui.available_width(), 800.0], // Large height to simulate overflow
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
                *spp1 = false;
            }
        });
    }
}
