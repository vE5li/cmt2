extern crate seamonkey;
extern crate sfml;

mod graphics;
mod input;
mod context;
mod core;
mod window;
mod editor;

use std::env::args;

use seamonkey::*;
use core::Instance;

fn main() {
    let mut instance = Instance::new(&args().collect());
    display!(instance.new_editor());

    while instance.has_open_windows() {
        instance.handle_input();
    }

    instance.close();
}
