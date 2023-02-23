use egui::{
    plot::{Legend, Line, Plot},
    Color32,
};
use synth_structs::{Context, Input, Player};

use crate::{App, Ui};

pub struct Graph {
    input: Input,
    samples: u32,
    millis: f64,
    buffer: Option<Vec<[f64; 2]>>,
    player: Option<Player>,
    waves: u32,
    known_intersect: Option<u32>,
}

impl Graph {
    pub fn new(input: impl Into<Input> + 'static) -> Self {
        let mut instance = Self {
            input: input.into(),
            samples: 5000,
            millis: 1000.0,
            buffer: None,
            player: None,
            waves: 1,
            known_intersect: None,
        };

        instance.view_waves(4);
        instance.enable_buffer(true);

        instance
    }

    pub fn run(self) {
        App::new(self).run();
    }

    fn get_points(&self) -> Vec<[f64; 2]> {
        if let Some(buffer) = &self.buffer {
            buffer.clone()
        } else {
            self.plot()
        }
    }

    fn get_rate(&self) -> u32 {
        let secs = self.millis / 1000.0;
        (self.samples as f64 / secs) as u32
    }

    fn plot(&self) -> Vec<[f64; 2]> {
        let mut input = self.input.clone();
        (0..=self.samples)
            .into_iter()
            .map(|i| {
                [
                    self.millis / self.samples as f64 * i as f64,
                    input
                        .sample(Context {
                            rate: self.get_rate(),
                        })
                        .unwrap_or(0.0) as f64,
                ]
            })
            .collect()
    }

    fn set_playing(&mut self, playing: bool) {
        if playing {
            self.player = Some(Player::play_threaded(self.input.clone()));
        } else if self.player.is_some() {
            self.player = None;
        }
    }

    fn buffer_enabled(&self) -> bool {
        self.buffer.is_some()
    }

    fn enable_buffer(&mut self, enabled: bool) {
        if enabled {
            self.buffer = Some(self.plot());
        } else {
            self.buffer = None;
        }
    }

    fn update_buffer(&mut self) {
        if self.buffer_enabled() {
            self.buffer = Some(self.plot());
        }
    }

    fn est_intersects(&mut self) -> Vec<u32> {
        let points = self.get_points();
        let mut intersects = Vec::<u32>::new();
        let mut last: Option<f64> = None;

        for (i, point) in points.into_iter().map(|i| i[1]).enumerate() {
            if let Some(last) = last {
                if last.signum() != point.signum() {
                    intersects.push(i as u32);

                    if i == 0 {
                        self.known_intersect = Some(i as u32);
                    }
                }
            }

            last = Some(point)
        }

        if intersects.is_empty() {
            if let Some(intersect) = self.known_intersect {
                intersects.push(intersect);
            }
        }

        intersects
    }

    fn view_waves(&mut self, amount: u32) {
        self.waves = amount;
        if let Some(index) = self.est_intersects().first() {
            self.millis = self.millis * *index as f64 * amount as f64 / self.samples as f64;
            self.update_buffer();
        }
    }
}

impl Ui for Graph {
    fn title(&self) -> &str {
        "Graph"
    }

    fn view(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("line").legend(Legend::default()).show(ui, |ui| {
                ui.line({
                    Line::new(self.get_points())
                        .color(Color32::LIGHT_GREEN)
                        .name("plot")
                })
            });
        });

        egui::Window::new("Graph").show(ctx, |ui| {
            egui::Grid::new("")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    {
                        ui.label("plotted millis");
                        if ui
                            .add(egui::DragValue::new(&mut self.millis).clamp_range(1..=u32::MAX))
                            .changed()
                        {
                            self.update_buffer()
                        }
                        ui.end_row();
                    }

                    {
                        ui.label("samples");
                        if ui
                            .add(egui::DragValue::new(&mut self.samples).clamp_range(3..=u32::MAX))
                            .changed()
                        {
                            self.update_buffer()
                        }
                        ui.end_row();
                    }

                    {
                        if ui
                            .add(egui::Checkbox::new(&mut self.buffer_enabled(), "buffer"))
                            .changed()
                        {
                            self.enable_buffer(!self.buffer_enabled())
                        };
                        ui.end_row();
                    }

                    {
                        ui.label("plotted intersections");
                        if ui
                            .add(egui::DragValue::new(&mut self.waves).clamp_range(1..=u32::MAX))
                            .changed()
                        {
                            self.view_waves(self.waves);
                        }

                        ui.end_row();
                    }
                });
        });

        egui::Window::new("Player").show(ctx, |ui| {
            egui::Grid::new("")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    if ui
                        .add(egui::Checkbox::new(&mut self.player.is_some(), "play"))
                        .changed()
                    {
                        self.set_playing(self.player.is_none())
                    }
                    ui.end_row();
                });
        });
    }
}
