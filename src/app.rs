use std::string;

use egui::Vec2;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

   // #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    a: f32,
    b: f32,
    c: f32,
    result: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            a: 1.0,
            b: 1.0,
            c: 0.0,
            result: "".to_string(),
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
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Standard Form Solver:");

            ui.add(egui::DragValue::new(&mut self.a));

            ui.add(egui::DragValue::new(&mut self.b));

            ui.add(egui::DragValue::new(&mut self.c));

            if ui.button("Find Answer").clicked() {
                self.result = "".to_string();

            let mut vertex = Vec2{x: 0.0, y: 0.0};

            vertex.x = match self.a {
                    0.0 => 0.0,
                    _ => self.b * -1.0 / (2.0 * self.a),
                };
            vertex.y = (self.a* (vertex.x * vertex.x) + self.b* vertex.x + self.c);
            self.result += &format!("\nVertex: ({}, {})\n\n", vertex.x, vertex.y);
            self.result += &format!("Axis of Symmetry: X = {:#?}\n\n", vertex.x);
            self.result += &format!("Y - Intercept: (0, {:#?})\n\n", self.c);
            self.result += &format!("Y - Intercept (Mirrored across A. of S.): ({:#?}, {:#?})\n\n", vertex.x * 2.0, self.c);

            let plus_minus: String = if -vertex.x >= 0.0 {
                format!("+ {}", -vertex.x)
            } else {
                format!("{}", -vertex.x)
            };

            let plus_minus_y: String = if vertex.y >= 0.0 {
                format!("+ {}", vertex.y)
            } else {
                format!("{}", vertex.y)
            };
            

            self.result += &format!("Vertex Form: y = {:#?}(x {})^2 {}\n\n", self.a, plus_minus, plus_minus_y);

            let max_min: String = if self.a <= 0.0 { "Maximum".to_string()} else {"Minimum".to_string()};

            self.result += &format!("{} of {}\n\n", max_min, vertex.y);

        }
            ui.add(egui::Label::new(&self.result));


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
