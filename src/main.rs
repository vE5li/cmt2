extern crate seamonkey;
extern crate sfml;

#[cfg(feature = "debug")]
extern crate chrono;

#[cfg(feature = "debug")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "debug")]
#[macro_use]
mod debug;

mod input;
mod themes;
mod selection;
mod filebuffer;
mod elements;
mod managers;
mod dialogues;
mod interface;
mod system;

use std::env::args;

use seamonkey::*;
use system::Instance;

fn main() {
    let mut instance = Instance::new(&args().collect());
    display!(instance.new_interface());

    while instance.has_open_windows() {
        instance.handle_input();
    }

    instance.close();
}
