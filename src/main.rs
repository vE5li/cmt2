extern crate seamonkey;
extern crate sfml;

mod input;
mod elements;
mod dialogues;
mod interface;
mod system;

use std::env::args;

use seamonkey::*;
use system::Instance;

fn main() {
    let mut instance = Instance::new(&args().collect());
    display!(instance.new_editor());

    while instance.has_open_windows() {
        instance.handle_input();
    }

    instance.close();
}
