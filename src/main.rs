use eframe::egui;

fn main() {
    eframe::run_native(
        "My egui App",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Ronny".to_string(),
            age: 28,
        }
    }
}

pub struct TestText<'a> {
    text: &'a str,
}

impl<'a> egui::Widget for TestText<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label(self.text)
    }
}

impl<'a> TestText<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            ui.horizontal(|ui| {
                let responce = ui.label("your name: ");
                responce.on_hover_ui(|ui| {
                    egui::show_tooltip(ui.ctx(), egui::Id::new("my_tooltip"), |ui| {
                        ui.label("ASDASDA");
                    });
                });
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(TestText::new("Test"));

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Click each year").clicked() {
                self.age += 1;
            }

            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
