use crate::rustbook_code_blocks::{self};
use eframe::egui;

pub fn side_panel_ui(ctx: &egui::Context, spp1: &mut bool) {
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

            let code = rustbook_code_blocks::VARIABLES_MUTABILITY_DATATYPES;

            egui::ScrollArea::vertical()
                .max_height(600.0) // Limit height for scrollability
                .show(ui, |ui| {
                    ui.add_sized(
                        [ui.available_width(), 800.0], // Large height to simulate overflow
                        egui::TextEdit::multiline(&mut code.to_string()),
                    );
                });
            if ui.button("❌ Close").clicked() {
                *spp1 = false;
            }
        });
    }
}
