use crate::side_panel::{self};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    counter: i32,
    name_input: String,
    x: i32,
    show_popup_panel1: bool,
    first_frame: bool,
    language: String,
    code: String,

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
            show_popup_panel1: false,
            first_frame: true,
            language: "rs".into(),
            code: String::new(),
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

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn reset(&mut self) {
        self.counter = 0;
    }

    fn printx(&mut self) {
        let x = self.x;
        println!("the value of x is {}", x);
    }

    // fn load_image_to_texture(ctx: &egui::Context, path: &str) -> egui::TextureHandle {
    // let image = image::open(path).expect("Failed to load image");

    // let size = [image.width() as usize, image.height() as usize];
    // let image_buffer = image.to_rgba8(); // Converts to RGBA8

    // let pixels = image_buffer
    //     .pixels()
    //     .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
    //     .collect();

    // let color_image = egui::ColorImage {
    //     size,
    //     pixels,
    // };

    // ctx.load_texture("my_image", color_image, egui::TextureOptions::default())
    // }
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
        if self.first_frame {
            ctx.set_visuals(egui::Visuals::light());
            self.first_frame = false;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        side_panel::side_panel_ui(ctx, &mut self.show_popup_panel1);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("rust_eframe_egui project, a learning project following along and building with the Rust book. Find the Rust Book Below!");

            ui.hyperlink_to("The Rust Programming Language book", "https://doc.rust-lang.org/stable/book");

            egui::Frame::new().fill(egui::Color32::RED).show(ui, |ui| {
                ui.label("Label with red background");
            });

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

        egui::Area::new(egui::Id::new("code_area1")).movable(true).show(ctx, |ui| {
                ui.label("Drag and Reposition an Area using the 'Rustferris' image in the top left of each area, or anywhere else on an Area it seems! just not on text. The position of an area persists from where it was last dragged and positioned");
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/Rustferris2.gif"))
                        .fit_to_exact_size(egui::vec2(50.0, 50.0))
                        .corner_radius(5),
                );

        ui.horizontal(|ui| {
                ui.label("Language:");
                ui.text_edit_singleline(&mut self.language);
            });
        ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Syntax highlighting powered by ");
                ui.hyperlink_to("syntect", "https://github.com/trishume/syntect");
                ui.label(".");

            });

        let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
        ui.collapsing("Theme", |ui| {
            ui.group(|ui| {
                theme.ui(ui);
                theme.clone().store_in_memory(ui.ctx());
            });
        });
        let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
            let mut layout_job = egui_extras::syntax_highlighting::highlight(
                ui.ctx(),
                ui.style(),
                &theme,
                text,
                &mut self.language,
            );
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };


                ui.label("Here are Rust principles on Variables and Mutability & DataTypes in Rust");
                let code = r#"
//Variable and Mutability & DataTypes in Rust

// This code would not compile, because by default, variables are immutable, and as the RustBook states, 'When a variable is immutable, once a value is bound to a name, you cant change that value.'

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
"#;

                ui.add(
                    egui::TextEdit::multiline(&mut code.to_string())
                        .font(egui::TextStyle::Monospace) // Use monospaced font
                        .code_editor() // Enables code styling
                        .desired_rows(13)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter),
                );
            });

        egui::Area::new(egui::Id::new("my_area2"))
            .movable(true)
            .show(ctx, |ui| {
                egui::Frame::new() // Start with no built-in padding/frame
                    .stroke(egui::Stroke::new(1.0, egui::Color32::ORANGE)) // border width and color
                    // .fill(egui::Color32::from_gray(30)) // optional background fill
                    .corner_radius(egui::CornerRadius::same(1)) // optional rounded corners
                    .show(ui, |ui| {
                        ui.add(
                    egui::Image::new(egui::include_image!("../assets/Rustferris2.gif"))
                        .fit_to_exact_size(egui::vec2(50.0, 50.0))
                        .corner_radius(5),
                        );
                        ui.label("Cool Floating Area with some ui components and using egui::Frame for a border. egui::Area itself does not directly control its size — it wraps content and takes the size of whatever UI elements are inside it.");
                        ui.heading("Hello, eframe + egui!");

                        ui.horizontal(|ui| {
                            ui.label("Enter your name:");
                            ui.text_edit_singleline(&mut self.name_input);
                        });

                        ui.separator();

                        if ui.button("Increment").clicked() {
                            self.increment();
                        }

                        if ui.button("Reset").clicked() {
                            self.reset();
                        }

                        if ui.button("PrintLn").clicked() {
                            self.printx();
                        }

                        ui.label(format!("Counter value: {}", self.counter));

                        if !self.name_input.is_empty() {
                            ui.label(format!("Welcome, {}!", self.name_input));
                        }
                    });
            });

            egui::Area::new(egui::Id::new("my_area"))
            .movable(true)
            .show(ctx, |ui| {
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/Rustferris2.gif"))
                        .fit_to_exact_size(egui::vec2(50.0, 50.0))
                        .corner_radius(5),
                );
                ui.label("Cool Floating Rustacean in an egui::Area. Areas are dragable, click the center and hold and drag area around to desired position. egui::Area itself does not directly control its size — it wraps content and takes the size of whatever UI elements are inside it.");

                ui.add(
                    egui::Image::new(egui::include_image!("../assets/Rust_logo_animated1.gif"))
                        .fit_to_exact_size(egui::vec2(200.0, 200.0))
                        .corner_radius(5),
                );
            });

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
