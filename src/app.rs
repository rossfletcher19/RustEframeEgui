use crate::central_panel::{self};
use crate::side_panel::{self};
use crate::top_panel::{self};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Define your app state here.
    first_frame: bool,
    side_panel_state: side_panel::SidePanelState,
    central_panel_state: central_panel::CentralPanelState,
    // This is how you opt-out of serialization of a field
    // #[serde(skip)]
    // value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            /*
            Example stuff:
            */
            first_frame: true,
            side_panel_state: side_panel::SidePanelState::new(),
            central_panel_state: central_panel::CentralPanelState::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // let mut style = (*ctx.style()).clone();
        // style.visuals.override_text_color = Some(Color32::from_rgb(183, 65, 14));
        // ctx.set_style(style);

        if self.first_frame {
            ctx.set_visuals(egui::Visuals::light());
            self.first_frame = false;
        }

        top_panel::top_panel_ui(ctx);

        side_panel::side_panel_ui(ctx, &mut self.side_panel_state);

        central_panel::central_panel_ui(ctx, &mut self.central_panel_state);
    }
}
