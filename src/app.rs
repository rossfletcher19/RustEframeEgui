use crate::central_panel::{self};
use crate::side_panel::{self};
use crate::top_panel::{self};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    counter: i32,
    name_input: String,
    x: i32,
    first_frame: bool,
    language: String,
    code: String,
    show_area1: bool,
    show_area2: bool,
    show_area3: bool,
    side_panel_state: side_panel::SidePanelState,

    // This is how you opt-out of serialization of a field
    #[serde(skip)]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            /*
            Example stuff:
            */
            label: "Hello Worldddd!!!".to_owned(),
            value: 3.5,
            counter: 0,
            name_input: String::new(),
            x: 5,
            first_frame: true,
            language: "rs".into(),
            code: String::new(),
            show_area1: false,
            show_area2: false,
            show_area3: false,
            side_panel_state: side_panel::SidePanelState::new(),
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

        central_panel::central_panel_ui(
            ctx,
            &mut self.value,
            &mut self.label,
            &mut self.language,
            &mut self.name_input,
            &mut self.counter,
            &mut self.x,
            &mut self.show_area1,
            &mut self.show_area2,
            &mut self.show_area3,
        );
    }
}
