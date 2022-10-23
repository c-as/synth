use synth::ocils::Sine;
use synth_ui::Graph;

fn main() {
    Graph::new(Sine::new(100) * 0.5).run();
}
