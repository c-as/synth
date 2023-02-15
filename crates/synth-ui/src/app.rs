use eframe::NativeOptions;

use crate::Ui;

pub struct App<T>(T);

impl<T: Ui + 'static> App<T> {
    pub fn new(ui: T) -> Self {
        Self(ui)
    }

    pub fn run(self) {
        let options = NativeOptions {
            initial_window_size: Some(egui::vec2(1280.0, 720.0)),
            centered: true,
            ..Default::default()
        };

        eframe::run_native(
            self.0.title().to_owned().as_str(),
            options,
            Box::new(|cc| {
                cc.egui_ctx.set_pixels_per_point(1.5);
                Box::new(self)
            }),
        )
        .ok();
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
