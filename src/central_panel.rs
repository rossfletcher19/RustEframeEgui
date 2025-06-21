use eframe::egui;
use egui::RichText;

fn increment(counter: &mut i32) {
    *counter += 1;
}

fn reset(counter: &mut i32) {
    *counter = 0;
}

fn printx(x: &mut i32) {
    let x = *x;
    println!("the value of x is {}", x);
}

pub fn central_panel_ui(
    ctx: &egui::Context,
    value: &mut f32,
    label: &mut String,
    language: &mut String,
    name_input: &mut String,
    counter: &mut i32,
    x: &mut i32,
    show_area1: &mut bool,
    show_area2: &mut bool,
    show_area3: &mut bool,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui|{
            ui.add(
                    egui::Image::new(egui::include_image!("../assets/Rust_logo_animated1.gif"))
                        .fit_to_exact_size(egui::vec2(50.0, 50.0))
                        .corner_radius(5),
                );
                ui.heading(RichText::new("rust_eframe_egui project, a learning project following along and building with the Rust book. Find the Rust Book Below!")
                    .color(egui::Color32::from_rgb(183, 65, 14))
                    .size(18.0)
                );
        });

            let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());

            ui.hyperlink_to("The Rust Programming Language book", "https://doc.rust-lang.org/stable/book");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            if label.is_empty() {

            } else {
                ui.label(format!("Your text shows here: {}", label));
            }

            ui.separator();

            ui.horizontal(|ui| {

                ui.hyperlink_to("eframe source code.", "https://github.com/emilk/eframe_template");
                if ui.button("Toggle Area 1").clicked() {
                    *show_area1 = !*show_area1;
                }
                if ui.button("Toggle Area 2").clicked() {
                    *show_area2 = !*show_area2;
                }
                if ui.button("Toggle Area 3").clicked() {
                    *show_area3 = !*show_area3;
                }

            ui.label("Language:");
                ui.add_enabled_ui(false, |ui|{
                    ui.text_edit_singleline(language);
                });
                ui.collapsing("Theme", |ui| {
                    ui.group(|ui| {
                        theme.ui(ui);
                        theme.clone().store_in_memory(ui.ctx());
                    });
                });
            });

            if *show_area1 {
            egui::Area::new(egui::Id::new("code_area1")).movable(true).show(ctx, |ui| {
                ui.label("Drag and Reposition an Area using the 'Rustferris' image in the top left of each area, or anywhere else on an Area it seems! just not on text. The position of an area persists from where it was last dragged and positioned");
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/Rustferris2.gif"))
                        .fit_to_exact_size(egui::vec2(50.0, 50.0))
                        .corner_radius(5),
                );

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
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Syntax highlighting powered by ");
                    ui.hyperlink_to("syntect", "https://github.com/trishume/syntect");
                    ui.label(".");
                });
            });
        }

        if *show_area2 {
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
                            ui.text_edit_singleline(name_input);
                        });

                        ui.separator();

                        if ui.button("Increment").clicked() {
                            increment(counter);
                        }

                        if ui.button("Reset").clicked() {
                            reset(counter);
                        }

                        if ui.button("PrintLn").clicked() {
                            printx(x);
                        }

                        ui.label(format!("Counter value: {}", counter));

                        if name_input.is_empty() {

                        } else {
                            ui.label(format!("Welcome, {}!", name_input));
                        }
                    });
            });

        }

        if *show_area3 {
            egui::Area::new(egui::Id::new("my_area"))
            .movable(true)
            .show(ctx, |ui| {
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/Rustferris2.gif"))
                        .fit_to_exact_size(egui::vec2(50.0, 50.0))
                        .corner_radius(5),
                );

                ui.label("Cool Floating Rustacean in an egui::Area. Areas are dragable, Drag and Reposition an Area using the 'Rustferris' image in the top left of each area, or anywhere else on an Area it seems! just not on text. The position of an area persists from where it was last dragged and positioned. click the center and hold and drag area around to desired position. egui::Area itself does not directly control its size — it wraps content and takes the size of whatever UI elements are inside it.");
                egui::Frame::new().fill(egui::Color32::WHITE).show(ui, |ui| {
                    ui.label(RichText::new("Label with white background, Rust hex color")
                        .color(egui::Color32::from_rgb(183, 65, 14))
                        .size(16.0)
                    );
                });


            });
        }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });


        });

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
}
