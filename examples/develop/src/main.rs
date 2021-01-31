#![feature(async_closure)]
#![feature(box_syntax)]

mod entry;

use entry::engine_entry;

fn main() {
    engine_entry();
}