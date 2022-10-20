pub mod app;
pub mod graph;

pub use app::App;
pub use graph::Graph;

pub trait Ui {
    fn view(&mut self, ctx: &egui::Context);
    fn title(&self) -> &str;
}
