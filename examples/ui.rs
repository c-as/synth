use synth::ocils::Sine;
use synth_egui::Graph;

fn main() {
    Graph::new(Sine::new(100) * 0.5).run();
}
