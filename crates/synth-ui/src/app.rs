use eframe::NativeOptions;

use crate::Ui;

pub struct App<T>(T);

impl<T: Ui + 'static> App<T> {
    pub fn new(ui: T) -> Self {
        Self(ui)
    }

    pub fn run(self) {
        eframe::run_native(
            self.0.title().to_owned().as_str(),
            NativeOptions::default(),
            Box::new(|_| Box::new(self)),
        ).ok();
    }
}

impl<T: Ui> eframe::App for App<T> {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label("Synth");
                ui.separator();
                ui.label(self.0.title());
            })
        });

        self.0.view(ctx);
    }
}
