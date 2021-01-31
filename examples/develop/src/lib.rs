/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#![feature(box_syntax)]
#![feature(async_closure)]

// RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build --target=web
cfg_if::cfg_if!{if #[cfg(target_arch = "wasm32")] {


mod entry;

use wasm_bindgen::prelude::*;
use entry::engine_entry;

#[wasm_bindgen(start)]
pub fn main() {
    engine_entry();
}

}}