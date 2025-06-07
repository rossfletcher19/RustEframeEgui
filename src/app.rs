/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    counter: i32,
    name_input: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    x: i32,
    show_popup: bool,
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
            show_popup: false,
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
        egui::SidePanel::left("side_nav")
            .exact_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Sidebar");
                if ui
                    .button("📦 Variable and Mutability & DataTypes")
                    .clicked()
                {
                    self.show_popup = true;
                }
            });

        if self.show_popup {
            egui::Area::new(egui::Id::new("popup_panel"))
                // .fixed_pos(egui::pos2(200.0, 500.0)) // offset from sidebar
                .movable(true)
                .show(ctx, |ui| {egui::Frame::popup(ui.style())
                .fill(egui::Color32::from_gray(30))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                        .show(ui, |ui| {
                            ui.label("This is a popup panel!");
                            let code = r#"
//Variable and Mutability & DataTypes in Rust

// This code would not compile, because by default, variables are immutable, and as the RustBook states, 'When a variable is immutable, once a value is bound to a name, you cant change that value.'
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

Good code that would compile bc value is declared mutable,

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// ahhh Data Types in Rust, they are beautiful and we know exactly what the are, always. That feels good doesnt it?

Look at two data types in Rust: Scalar and compound

Scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. similar to other programming languages.

// Integers


"#;

                ui.add(
                    egui::TextEdit::multiline(&mut code.to_string())
                        .font(egui::TextStyle::Monospace) // Use monospaced font
                        .code_editor() // Enables code styling
                        // .desired_rows(13)
                        .lock_focus(true)
                        .desired_width(400.0),
                );
                            if ui.button("❌ Close").clicked() {
                                self.show_popup = false;
                            }
                        });
                });
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

        egui::Area::new(egui::Id::new("code_area1"))
            .movable(true)
            // .default_pos(egui::pos2(850.0, 300.0))
            .show(ctx, |ui| {
                ui.label("Drag and Reposition an Area using the 'Rustferris' image in the top left of each area, or anywhere else on an Area it seems! just not on text. The position of an area persists from where it was last dragged and positioned");
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/Rustferris2.gif"))
                        .fit_to_exact_size(egui::vec2(50.0, 50.0))
                        .corner_radius(5),
                );
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
                        .desired_width(400.0),
                );
            });

        egui::Area::new(egui::Id::new("my_area2"))
            .movable(true)
            // .default_pos(egui::pos2(275.0, 400.0))
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
            // .default_pos(egui::pos2(275.0, 600.0))
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

            // egui::Resize::default()
            //     .id_salt("resizable_frame") // unique id
            //     .default_size(self.size)
            //     .show(ui, |ui| {
            //         egui::Frame::new().fill(egui::Color32::RED).show(ui, |ui| {
            //             ui.label("Label with red background");
            //         });
            //     });

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
