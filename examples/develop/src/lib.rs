#[cfg(target_arch = "wasm32")]
mod engine;

#[cfg(target_arch = "wasm32")]
use butterscotch::run_event_loop;

#[cfg(target_arch = "wasm32")]
use engine::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    run_event_loop(Engine::new());
}