mod engine;

use butterscotch::run_event_loop;
use engine::*;

fn main() {
    run_event_loop(Engine::new());
}