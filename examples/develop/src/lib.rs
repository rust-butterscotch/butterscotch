// RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build --target=web
cfg_if::cfg_if!{if #[cfg(target_arch = "wasm32")] {

mod engine;

use wasm_bindgen::prelude::*;
use engine::engine_entry;

#[wasm_bindgen(start)]
pub fn main() {
    engine_entry();
}

}}